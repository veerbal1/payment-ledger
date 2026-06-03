use crate::amount::Amount;

pub enum LedgerDirection {
    Credit,
    Debit,
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
}
