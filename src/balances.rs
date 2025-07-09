use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum PalletError {
    InsufficientFunds,
}

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

const TRANSACTION_FEE: u128 = 10; // Let's say 10 units of currency

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        sender: &String,
        recipient: &String,
        amount: u128,
    ) -> Result<(), PalletError> {
        let sender_balance = self.balance(sender);
        let total_deduction = amount + TRANSACTION_FEE;

        if sender_balance < total_deduction {
            return Err(PalletError::InsufficientFunds);
        }

        let new_sender_balance = sender_balance - total_deduction;
        self.set_balance(sender, new_sender_balance);

        let recipient_balance = self.balance(recipient);
        let new_recipient_balance = recipient_balance + amount;
        self.set_balance(recipient, new_recipient_balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100); // Fixed assertion value
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_success() {
        let mut balances = super::Pallet::new();
        balances.set_balance(&"alice".to_string(), 100);
        balances.set_balance(&"bob".to_string(), 50);

        let transfer_result = balances.transfer(&"alice".to_string(), &"bob".to_string(), 30);
        assert!(transfer_result.is_ok());

        assert_eq!(balances.balance(&"alice".to_string()), 60);
        assert_eq!(balances.balance(&"bob".to_string()), 80);
    }

    #[test]
    fn transfer_insufficient_funds() {
        let mut balances = super::Pallet::new();
        balances.set_balance(&"alice".to_string(), 20); // Alice has 20

        let transfer_result = balances.transfer(&"alice".to_string(), &"bob".to_string(), 15);
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result, Err(PalletError::InsufficientFunds));

        assert_eq!(balances.balance(&"alice".to_string()), 20);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_exact_amount_with_fee() {
        let mut balances = super::Pallet::new();
        balances.set_balance(&"alice".to_string(), 40); // Alice has 40
        balances.set_balance(&"bob".to_string(), 0);

        let transfer_result = balances.transfer(&"alice".to_string(), &"bob".to_string(), 30);
        assert!(transfer_result.is_ok());

        assert_eq!(balances.balance(&"alice".to_string()), 0); // 40 - 40 = 0
        assert_eq!(balances.balance(&"bob".to_string()), 30); // 0 + 30 = 30
    }
}
