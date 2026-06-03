use crate::amount::Amount;

pub enum LedgerDirection {
    Credit,
    Debit,
}

impl LedgerDirection {
    pub fn sign(&self) -> i64 {
        match self {
            LedgerDirection::Credit => 1,
            LedgerDirection::Debit => -1,
        }
    }
}

pub struct LedgerEntry {
    id: String,
    account_id: String,
    amount: Amount,
    direction: LedgerDirection,
    transfer_id: String,
}

impl LedgerEntry {
    pub fn new(
        account_id: String,
        amount: Amount,
        direction: LedgerDirection,
        transfer_id: String,
    ) -> Self {
        let id = format!("{:x}", rand::random::<u64>());
        Self {
            id,
            account_id,
            amount,
            direction,
            transfer_id,
        }
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn direction(&self) -> &LedgerDirection {
        &self.direction
    }

    pub fn set_transfer_id(&mut self, transfer_id: String) {
        self.transfer_id = transfer_id
    }

    pub fn account_id(&self) -> &String {
        &self.account_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_ids() {
        let e1 = LedgerEntry::new(
            "a".into(),
            Amount::new(50).unwrap(),
            LedgerDirection::Credit,
            "t".into(),
        );
        let e2 = LedgerEntry::new(
            "a".into(),
            Amount::new(50).unwrap(),
            LedgerDirection::Credit,
            "t".into(),
        );
        assert_ne!(e1.id, e2.id);
    }

    #[test]
    fn fields_populated_correctly() {
        let amount = Amount::new(100).unwrap();
        let entry = LedgerEntry::new(
            "acc-1".into(),
            amount,
            LedgerDirection::Debit,
            "tx-1".into(),
        );
        assert!(!entry.id.is_empty());
        assert_eq!(entry.amount, Amount::new(100).unwrap());
        assert_eq!(entry.account_id, "acc-1");
        assert_eq!(entry.transfer_id, "tx-1");
        assert!(matches!(entry.direction, LedgerDirection::Debit));
    }

    #[test]
    fn test_ledger_direction_positive() {
        let direction = LedgerDirection::Credit;
        assert_eq!(direction.sign(), 1);
    }

    #[test]
    fn test_ledger_direction_negative() {
        let direction = LedgerDirection::Debit;
        assert_eq!(direction.sign(), -1);
    }
}
