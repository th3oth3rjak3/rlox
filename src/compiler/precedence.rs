/// Precedence represents the order used to compile expressions.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Precedence(u8);

impl Precedence {
    /// No expression precedence.
    pub const NONE: Self = Self(0);

    /// Assignment precedence.
    pub const ASSIGNMENT: Self = Self(1);

    /// Logical-or precedence.
    #[allow(dead_code)]
    pub const OR: Self = Self(2);

    /// Logical-and precedence.
    #[allow(dead_code)]
    pub const AND: Self = Self(3);

    /// Equality precedence.
    pub const EQUALITY: Self = Self(4);

    /// Comparison precedence.
    pub const COMPARISON: Self = Self(5);

    /// Addition and subtraction precedence.
    pub const TERM: Self = Self(6);

    /// Multiplication and division precedence.
    pub const FACTOR: Self = Self(7);

    /// Unary operator precedence.
    pub const UNARY: Self = Self(8);

    /// Function-call and property-access precedence.
    #[allow(dead_code)]
    pub const CALL: Self = Self(9);

    /// Primary-expression precedence.
    pub const PRIMARY: Self = Self(10);

    /// Returns the next higher expression precedence.
    pub fn next(self) -> Self {
        let n = self.0 + 1;
        Precedence(n.min(Self::PRIMARY.0))
    }

    /// Returns whether this precedence permits assignment.
    pub fn allows_assignment(self) -> bool {
        self <= Self::ASSIGNMENT
    }
}
