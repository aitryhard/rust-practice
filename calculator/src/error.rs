use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CalcError {
    ParseError(String),
    EvalError(String),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::ParseError(msg) => write!(f, "parse error: {}", msg),
            CalcError::EvalError(msg) => write!(f, "evaluation error: {}", msg),
        }
    }
}

impl std::error::Error for CalcError {}
