use crate::ast::HistoryIndexKind;

#[derive(Debug, thiserror::Error)]
pub enum ArithmeticError {
    #[error("overflow")]
    Overflow,
    #[error("underflow")]
    Underflow,
    #[error("attempt to divide by 0")]
    DivideBy0,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Arithmetic(#[from] ArithmeticError),
    #[error("parsing: {0}")]
    Parse(#[from] ParseValueError),
    #[error("{0:?} history index {1} out of bounds: [0..{2})")]
    HistoryOOB(HistoryIndexKind, usize, usize),
    #[error("attempted to perform an operation which only makes sense for integers, but value is currently a float")]
    ImproperlyFloat,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseValueError {
    #[error("\"{0}\" cannot be parsed as Value")]
    Simple(String),
    #[error("\"{0}\" cannot be parsed as Value given radix {1}")]
    Radix(String, u32),
}
