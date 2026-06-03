use serde::{Deserialize, Serialize};

use crate::entry::LedgerEntry;
use crate::errors::ErrorCode;
use crate::transfer::compute_balance;

#[derive(Deserialize, Serialize)]
pub struct CreateTransferRequest {
    pub from_account_id: String,
    pub to_account_id: String,
    pub amount: i64,
}

#[derive(Serialize)]
pub struct CreateTransferResponse {
    pub status: String,
    pub transfer_id: Option<String>,
    pub error: Option<String>,
    pub retryable: Option<bool>,
}

impl CreateTransferRequest {
    pub fn validate(&self) -> Result<(), ErrorCode> {
        if self.from_account_id.is_empty() || self.to_account_id.is_empty() {
            return Err(ErrorCode::ValidationError);
        }
        if self.amount < 0 {
            return Err(ErrorCode::InvalidAmount);
        }
        Ok(())
    }

    pub fn validate_business(&self, entries: &[LedgerEntry]) -> Result<(), ErrorCode> {
        if self.from_account_id == self.to_account_id {
            return Err(ErrorCode::SelfTransfer);
        }
        if self.amount == 0 {
            return Err(ErrorCode::ZeroAmount);
        }
        let balance = compute_balance(entries, &self.from_account_id);
        if balance < self.amount {
            return Err(ErrorCode::InsufficientFunds);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amount::Amount;
    use crate::entry::LedgerDirection;

    #[test]
    fn rejects_empty_from_account() {
        let req = CreateTransferRequest {
            from_account_id: String::new(),
            to_account_id: "acc-2".into(),
            amount: 100,
        };
        assert_eq!(req.validate(), Err(ErrorCode::ValidationError));
    }

    #[test]
    fn rejects_empty_to_account() {
        let req = CreateTransferRequest {
            from_account_id: "acc-1".into(),
            to_account_id: String::new(),
            amount: 100,
        };
        assert_eq!(req.validate(), Err(ErrorCode::ValidationError));
    }

    #[test]
    fn rejects_negative_amount() {
        let req = CreateTransferRequest {
            from_account_id: "acc-1".into(),
            to_account_id: "acc-2".into(),
            amount: -50,
        };
        assert_eq!(req.validate(), Err(ErrorCode::InvalidAmount));
    }

    #[test]
    fn rejects_all_empty() {
        let req = CreateTransferRequest {
            from_account_id: String::new(),
            to_account_id: String::new(),
            amount: 0,
        };
        assert_eq!(req.validate(), Err(ErrorCode::ValidationError));
    }

    #[test]
    fn valid_request_passes() {
        let req = CreateTransferRequest {
            from_account_id: "acc-1".into(),
            to_account_id: "acc-2".into(),
            amount: 100,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn rejects_self_transfer() {
        let req = CreateTransferRequest {
            from_account_id: "acc-1".into(),
            to_account_id: "acc-1".into(),
            amount: 100,
        };
        assert_eq!(req.validate_business(&[]), Err(ErrorCode::SelfTransfer));
    }

    #[test]
    fn rejects_zero_amount() {
        let req = CreateTransferRequest {
            from_account_id: "acc-1".into(),
            to_account_id: "acc-2".into(),
            amount: 0,
        };
        assert_eq!(req.validate_business(&[]), Err(ErrorCode::ZeroAmount));
    }

    #[test]
    fn rejects_overdraft() {
        let credit = LedgerEntry::new(
            "acc-1".into(),
            Amount::new(100).unwrap(),
            LedgerDirection::Credit,
            String::new(),
        );
        let req = CreateTransferRequest {
            from_account_id: "acc-1".into(),
            to_account_id: "acc-2".into(),
            amount: 200,
        };
        assert_eq!(
            req.validate_business(&[credit]),
            Err(ErrorCode::InsufficientFunds)
        );
    }

    #[test]
    fn allows_transfer_when_balance_sufficient() {
        let credit = LedgerEntry::new(
            "acc-1".into(),
            Amount::new(300).unwrap(),
            LedgerDirection::Credit,
            String::new(),
        );
        let req = CreateTransferRequest {
            from_account_id: "acc-1".into(),
            to_account_id: "acc-2".into(),
            amount: 200,
        };
        assert!(req.validate_business(&[credit]).is_ok());
    }
}
