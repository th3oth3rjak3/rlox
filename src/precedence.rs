/// Precedence is the order in which parsing must be
/// done to produce the correct expression evaluation.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    /// next returns the precedence level that is one higher
    /// than current.
    pub fn next(self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call | Precedence::Primary => Precedence::Primary,
        }
    }

    /// can_assign checks to see if the precedence is lower
    /// than assignment.
    pub fn can_assign(self) -> bool {
        (self as u8) <= (Self::Assignment as u8)
    }
}
