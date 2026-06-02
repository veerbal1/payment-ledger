use crate::{currency::Currency, errors::ErrorCode};

#[derive(Debug)]
pub struct Amount {
    value: i64,
    currency: Currency,
}

impl Amount {
    pub fn new(value: i64) -> Result<Self, ErrorCode> {
        if value < 0 {
            return Err(ErrorCode::InvalidAmount);
        }
        Ok(Self {
            value,
            currency: Currency::Usd,
        })
    }

    pub fn checked_add(&self, amount: &Amount) -> Result<Amount, ErrorCode> {
        if self.currency != amount.currency {
            return Err(ErrorCode::MismatchedCurrency);
        }
        let value = self
            .value
            .checked_add(amount.value)
            .ok_or(ErrorCode::MathError)?;
        Ok(Amount {
            value,
            currency: self.currency,
        })
    }

    pub fn checked_sub(&self, amount: &Amount) -> Result<Amount, ErrorCode> {
        if self.currency != amount.currency {
            return Err(ErrorCode::MismatchedCurrency);
        }
        let value = self
            .value
            .checked_sub(amount.value)
            .ok_or(ErrorCode::MathError)?;
        if value < 0 {
            return Err(ErrorCode::InvalidAmount);
        }
        Ok(Amount {
            value,
            currency: self.currency,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_amount_positive_pass() {
        let amount = Amount::new(10);
        let amount_unwrapped = amount.unwrap();
        assert_eq!(amount_unwrapped.value, 10);
        assert_eq!(amount_unwrapped.currency, Currency::Usd);
    }

    #[test]
    fn test_add_amount_zero_pass() {
        let amount = Amount::new(0);
        let amount_unwrapped = amount.unwrap();
        assert_eq!(amount_unwrapped.value, 0)
    }

    #[test]
    fn test_add_amount_negative_fail() {
        let amount = Amount::new(-10);
        assert!(amount.is_err());
        let err = amount.unwrap_err();
        assert_eq!(err, ErrorCode::InvalidAmount);
    }

    #[test]
    fn test_checked_add_pass() {
        let amount = Amount::new(10).unwrap();
        let amount_to_add = Amount::new(5).unwrap();
        let result = amount.checked_add(&amount_to_add);
        assert_eq!(result.unwrap().value, 15)
    }

    #[test]
    fn test_checked_sub_pass() {
        let amount = Amount::new(10).unwrap();
        let amount_to_add = Amount::new(5).unwrap();
        let result = amount.checked_sub(&amount_to_add);
        assert_eq!(result.unwrap().value, 5)
    }

    #[test]
    fn test_checked_sub_fail() {
        let amount = Amount::new(10).unwrap();
        let amount_to_add = Amount::new(15).unwrap();
        let result = amount.checked_sub(&amount_to_add);
        assert!(result.is_err())
    }

    #[test]
    fn test_overflow_error() {
        let amount = Amount::new(i64::MAX).unwrap();
        let one = Amount::new(1).unwrap();
        let result = amount.checked_add(&one);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::MathError);
    }
}
