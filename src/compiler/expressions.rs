use super::Compiler;
use super::parse_rule::ParseRule;
use super::precedence::Precedence;
use crate::common::Value;
use crate::opcode::*;
use crate::token::{Token, TokenKind};

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Compiles an expression using the lowest expression precedence.
    pub(super) fn compile_expression(&mut self) {
        self.compile_expression_with_precedence(Precedence::ASSIGNMENT);
    }

    /// Compiles a numeric literal into a bytecode constant.
    pub(super) fn compile_number_literal(&mut self, _: bool) {
        let lexeme = self.tokens.previous().lexeme(self.source);
        let value: f64 = lexeme
            .parse()
            .expect("Scanner produced invalid number literal.");
        self.emit_constant(Value::Number(value));
    }

    /// Compiles an expression enclosed in parentheses.
    pub(super) fn compile_grouping_expression(&mut self, _: bool) {
        self.compile_expression();
        self.consume_current_token(TokenKind::RightParen, "Expect ')' after expression.");
    }

    /// Compiles a unary expression.
    pub(super) fn compile_unary_expression(&mut self, _: bool) {
        let operator = self.tokens.previous().kind;

        self.compile_expression_with_precedence(Precedence::UNARY);

        match operator {
            TokenKind::Minus => self.emit_byte(OP_NEGATE),
            TokenKind::Bang => self.emit_byte(OP_NOT),
            _ => (),
        }
    }

    /// Compiles a binary expression.
    pub(super) fn compile_binary_expression(&mut self, _: bool) {
        let operator = self.tokens.previous().kind;
        let rule = self.parse_rule_for(operator);
        self.compile_expression_with_precedence(rule.precedence.next());

        match operator {
            TokenKind::BangEqual => self.emit_byte(OP_NOT_EQUAL),
            TokenKind::EqualEqual => self.emit_byte(OP_EQUAL),
            TokenKind::Greater => self.emit_byte(OP_GREATER),
            TokenKind::GreaterEqual => self.emit_byte(OP_GREATER_EQUAL),
            TokenKind::Less => self.emit_byte(OP_LESS),
            TokenKind::LessEqual => self.emit_byte(OP_LESS_EQUAL),
            TokenKind::Plus => self.emit_byte(OP_ADD),
            TokenKind::Minus => self.emit_byte(OP_SUBTRACT),
            TokenKind::Star => self.emit_byte(OP_MULTIPLY),
            TokenKind::Slash => self.emit_byte(OP_DIVIDE),
            _ => (),
        }
    }

    /// Compiles a literal value.
    pub(super) fn compile_literal(&mut self, _: bool) {
        match self.tokens.previous().kind {
            TokenKind::False => self.emit_byte(OP_FALSE),
            TokenKind::True => self.emit_byte(OP_TRUE),
            TokenKind::Nil => self.emit_byte(OP_NIL),
            _ => (),
        }
    }

    /// Compiles a string literal into a bytecode constant.
    pub(super) fn compile_string_literal(&mut self, _: bool) {
        let lexeme = self.tokens.previous().lexeme(self.source);
        let lexeme = lexeme[1..lexeme.len() - 1].to_owned();
        let id = self.heap.intern_string(lexeme);
        self.emit_constant(Value::Obj(id));
    }

    /// Compiles a variable expression.
    pub(super) fn compile_variable_expression(&mut self, can_assign: bool) {
        self.compile_named_variable(self.tokens.previous(), can_assign);
    }

    /// Compiles reading from or assigning to a named variable.
    pub(super) fn compile_named_variable(&mut self, name: Token, can_assign: bool) {
        let arg = self.add_identifier_constant(name);
        if can_assign && self.match_current_token(TokenKind::Equal) {
            self.compile_expression();
            self.emit_set_global(arg);
        } else {
            self.emit_get_global(arg);
        }
    }

    /// Compiles an expression using the provided precedence.
    pub(super) fn compile_expression_with_precedence(&mut self, precedence: Precedence) {
        self.advance_token();
        let prefix_rule = self.parse_rule_for(self.tokens.previous().kind).prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression.");
            return;
        }

        let can_assign = precedence.allows_assignment();
        prefix_rule.unwrap()(self, can_assign);

        while precedence <= self.parse_rule_for(self.tokens.current().kind).precedence {
            self.advance_token();
            let infix_rule = self.parse_rule_for(self.tokens.previous().kind).infix;
            infix_rule.expect("infix rule should not have been None")(self, can_assign);
        }

        if can_assign && self.match_current_token(TokenKind::Equal) {
            self.error("Invalid assignment target.");
        }
    }

    /// Returns the parse rule associated with the provided token kind.
    pub(super) fn parse_rule_for(&self, kind: TokenKind) -> ParseRule<'src, 'heap> {
        match kind {
            TokenKind::LeftParen => ParseRule::new(
                Some(Self::compile_grouping_expression),
                None,
                Precedence::NONE,
            ),
            TokenKind::Minus => ParseRule::new(
                Some(Self::compile_unary_expression),
                Some(Self::compile_binary_expression),
                Precedence::TERM,
            ),
            TokenKind::Plus => ParseRule::new(
                None,
                Some(Self::compile_binary_expression),
                Precedence::TERM,
            ),
            TokenKind::Slash => ParseRule::new(
                None,
                Some(Self::compile_binary_expression),
                Precedence::FACTOR,
            ),
            TokenKind::Star => ParseRule::new(
                None,
                Some(Self::compile_binary_expression),
                Precedence::FACTOR,
            ),
            TokenKind::Number => {
                ParseRule::new(Some(Self::compile_number_literal), None, Precedence::NONE)
            }
            TokenKind::True | TokenKind::False | TokenKind::Nil => {
                ParseRule::new(Some(Self::compile_literal), None, Precedence::NONE)
            }
            TokenKind::Bang => {
                ParseRule::new(Some(Self::compile_unary_expression), None, Precedence::NONE)
            }
            TokenKind::BangEqual | TokenKind::EqualEqual => ParseRule::new(
                None,
                Some(Self::compile_binary_expression),
                Precedence::EQUALITY,
            ),
            TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::Less
            | TokenKind::LessEqual => ParseRule::new(
                None,
                Some(Self::compile_binary_expression),
                Precedence::COMPARISON,
            ),
            TokenKind::String => {
                ParseRule::new(Some(Self::compile_string_literal), None, Precedence::NONE)
            }
            TokenKind::Identifier => ParseRule::new(
                Some(Self::compile_variable_expression),
                None,
                Precedence::NONE,
            ),
            _ => ParseRule::new(None, None, Precedence::NONE),
        }
    }
}
