use axum::http::StatusCode;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidAmount,
    MathError,

    MismatchedCurrency,
    UnbalancedTransfer,
    ValidationError,
    SelfTransfer,
    ZeroAmount,
    InsufficientFunds,
    IdempotencyKeyReused,
}

impl ErrorCode {
    pub fn is_retryable(&self) -> bool {
        matches!(self, ErrorCode::InsufficientFunds)
    }

    pub fn http_status(&self) -> StatusCode {
        match self {
            ErrorCode::InvalidAmount => StatusCode::BAD_REQUEST,
            ErrorCode::MathError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::MismatchedCurrency => StatusCode::BAD_REQUEST,
            ErrorCode::UnbalancedTransfer => StatusCode::BAD_REQUEST,
            ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            ErrorCode::SelfTransfer => StatusCode::BAD_REQUEST,
            ErrorCode::ZeroAmount => StatusCode::BAD_REQUEST,
            ErrorCode::InsufficientFunds => StatusCode::CONFLICT,
            ErrorCode::IdempotencyKeyReused => StatusCode::CONFLICT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insufficient_funds_is_retryable() {
        assert!(ErrorCode::InsufficientFunds.is_retryable());
    }

    #[test]
    fn insufficient_funds_status_409() {
        assert_eq!(
            ErrorCode::InsufficientFunds.http_status(),
            StatusCode::CONFLICT
        );
    }

    #[test]
    fn idempotency_key_reused_not_retryable() {
        assert!(!ErrorCode::IdempotencyKeyReused.is_retryable());
    }

    #[test]
    fn idempotency_key_reused_status_409() {
        assert_eq!(
            ErrorCode::IdempotencyKeyReused.http_status(),
            StatusCode::CONFLICT
        );
    }

    #[test]
    fn invalid_amount_not_retryable() {
        assert!(!ErrorCode::InvalidAmount.is_retryable());
    }

    #[test]
    fn invalid_amount_status_400() {
        assert_eq!(
            ErrorCode::InvalidAmount.http_status(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn math_error_not_retryable() {
        assert!(!ErrorCode::MathError.is_retryable());
    }

    #[test]
    fn math_error_status_500() {
        assert_eq!(
            ErrorCode::MathError.http_status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
