use super::Compiler;
use super::precedence::Precedence;

/// ParseFn is any function that can parse source.
pub(super) type ParseFn<'src, 'heap> = fn(&mut Compiler<'src, 'heap>, bool);

/// ParseRule is a rule that describes how to parse
/// precedences.
pub(super) struct ParseRule<'src, 'heap> {
    pub(super) prefix: Option<ParseFn<'src, 'heap>>,
    pub(super) infix: Option<ParseFn<'src, 'heap>>,
    pub(super) precedence: Precedence,
}

impl<'src, 'heap> ParseRule<'src, 'heap> {
    /// Create a new parse rule.
    pub const fn new(
        prefix: Option<ParseFn<'src, 'heap>>,
        infix: Option<ParseFn<'src, 'heap>>,
        precedence: Precedence,
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}
