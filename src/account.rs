use rand::distr::{Alphanumeric, SampleString};

use crate::currency::Currency;

type AccountId = String;

pub struct Account {
    id: AccountId,
    name: String,
    currency: Currency,
}

impl Account {
    pub fn new(name: String, currency: Currency) -> Self {
        let id: AccountId = Alphanumeric.sample_string(&mut rand::rng(), 16);
        Self { id, name, currency }
    }

    pub fn get_id(&self) -> AccountId {
        self.id.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_different_ids() {
        let account_a = Account::new("Sukh".to_string(), Currency::Usd);
        let account_b = Account::new("Veer".to_string(), Currency::Usd);
        assert_ne!(account_a.id, account_b.id);
    }

    #[test]
    fn test_account_populate_all_fields() {
        let account_a = Account::new("Sukh".to_string(), Currency::Usd);
        assert!(account_a.id.len() > 0);
        assert_eq!(account_a.name, "Sukh".to_string());
        assert_eq!(account_a.currency, Currency::Usd);
    }
}
