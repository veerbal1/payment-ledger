use crate::errors::ErrorCode;

#[derive(Debug)]
pub struct Amount {
    value: i64,
}

impl Amount {
    pub fn new(value: i64) -> Result<Self, ErrorCode> {
        if value < 0 {
            return Err(ErrorCode::InvalidAmount);
        }
        Ok(Self { value })
    }
    pub fn add_amount(&mut self, amount: i64) -> Result<i64, ErrorCode> {
        if amount <= 0 {
            return Err(ErrorCode::InvalidAmount);
        }
        self.value = self.value.checked_add(amount).ok_or(ErrorCode::MathError)?;
        Ok(self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_amount_positive_pass() {
        let amount = Amount::new(10);
        assert_eq!(amount.unwrap().value, 10)
    }

    #[test]
    fn test_add_amount_zero_pass() {
        let amount = Amount::new(0);
        assert_eq!(amount.unwrap().value, 0)
    }

    #[test]
    fn test_add_amount_negative_fail() {
        let amount = Amount::new(-10);
        assert!(amount.is_err());
        let err = amount.unwrap_err();
        assert_eq!(err, ErrorCode::InvalidAmount);
    }
}
