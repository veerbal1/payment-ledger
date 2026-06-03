use crate::{entry::LedgerEntry, errors::ErrorCode};

pub fn compute_balance(ledger_entries: &[LedgerEntry], account_id: &str) -> i64 {
    let filtered_entries: Vec<&LedgerEntry> = ledger_entries
        .iter()
        .filter(|entry| entry.account_id().to_string() == account_id)
        .collect();
    let sum: i64 = filtered_entries
        .iter()
        .map(|entry| entry.amount().value() * entry.direction().sign())
        .sum();
    sum
}

pub struct Transfer {
    id: String,
    entries: Vec<LedgerEntry>,
}

impl Transfer {
    pub fn new(mut entries: Vec<LedgerEntry>) -> Result<Transfer, ErrorCode> {
        let id = rand::random::<u64>().to_string();
        let sum: i64 = entries
            .iter()
            .map(|entry| {
                let amount = entry.amount();
                let direction = entry.direction().sign();
                let net = amount.value() * direction;
                net
            })
            .sum();
        if sum != 0 {
            return Err(ErrorCode::UnbalancedTransfer);
        };

        for entry in entries.iter_mut() {
            entry.set_transfer_id(id.clone());
        }

        let transfer = Transfer { id, entries };
        Ok(transfer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amount::Amount;
    use crate::entry::LedgerDirection;

    #[test]
    fn balanced_transfer_succeeds() {
        let credit = LedgerEntry::new(
            "acc-1".into(),
            Amount::new(500).unwrap(),
            LedgerDirection::Credit,
            String::new(),
        );
        let debit = LedgerEntry::new(
            "acc-2".into(),
            Amount::new(500).unwrap(),
            LedgerDirection::Debit,
            String::new(),
        );
        let transfer = Transfer::new(vec![credit, debit]);
        assert!(transfer.is_ok());
    }

    #[test]
    fn unbalanced_transfer_fails() {
        let credit = LedgerEntry::new(
            "acc-1".into(),
            Amount::new(500).unwrap(),
            LedgerDirection::Credit,
            String::new(),
        );
        let debit = LedgerEntry::new(
            "acc-2".into(),
            Amount::new(200).unwrap(),
            LedgerDirection::Debit,
            String::new(),
        );
        let result = Transfer::new(vec![credit, debit]);
        assert!(matches!(result, Err(ErrorCode::UnbalancedTransfer)));
    }

    #[test]
    fn balanced_transfer_varied_amounts() {
        let cases = [
            (100, "acc-1", "acc-2"),
            (1, "savings", "checking"),
            (1_000_000, "corporate", "vendor"),
            (0, "zero", "zero-too"),
        ];
        for &(amount, credit_acct, debit_acct) in &cases {
            let credit = LedgerEntry::new(
                credit_acct.into(),
                Amount::new(amount).unwrap(),
                LedgerDirection::Credit,
                String::new(),
            );
            let debit = LedgerEntry::new(
                debit_acct.into(),
                Amount::new(amount).unwrap(),
                LedgerDirection::Debit,
                String::new(),
            );
            let result = Transfer::new(vec![credit, debit]);
            assert!(
                result.is_ok(),
                "Failed for amount={}, credit={}, debit={}",
                amount,
                credit_acct,
                debit_acct
            );
        }
    }

    #[test]
    fn test_compute_balance() {
        use crate::amount::Amount;
        use crate::entry::LedgerDirection;

        let credit = LedgerEntry::new(
            "acc-1".into(),
            Amount::new(500).unwrap(),
            LedgerDirection::Credit,
            String::new(),
        );
        let debit = LedgerEntry::new(
            "acc-1".into(),
            Amount::new(200).unwrap(),
            LedgerDirection::Debit,
            String::new(),
        );
        let entries = vec![credit, debit];
        let balance = compute_balance(&entries, "acc-1");
        assert_eq!(balance, 300);
    }
}
