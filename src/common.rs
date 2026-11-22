use std::fmt::Display;

/// Value represents a lox runtime value.
#[derive(Debug, Clone, Copy)]
pub enum Value {
    Nil,
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => f.write_str("<nil>"),
            Self::Number(n) => f.write_fmt(format_args!("{n}")),
        }
    }
}
