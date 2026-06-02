#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidAmount,
    MathError,

    MismatchedCurrency
}
