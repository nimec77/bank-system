use std::collections::{hash_map, HashMap};

pub type Name = String;
type Balance = i64;

pub struct Storage {
    accounts: HashMap<Name, Balance>,
}

impl Storage {
    /// Создаёт новый пустой банк
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    // Internal methods used by UserManager and BalanceManager
    pub(crate) fn add_user_internal(&mut self, name: Name) -> Option<Balance> {
        if let hash_map::Entry::Vacant(e) = self.accounts.entry(name) {
            e.insert(0);
            Some(0)
        } else {
            None
        }
    }

    pub(crate) fn remove_user_internal(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name)
    }

    pub(crate) fn get_balance_internal(&self, name: &Name) -> Option<Balance> {
        self.accounts.get(name).copied()
    }

    pub(crate) fn deposit_internal(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            *balance += amount;
            Ok(())
        } else {
            Err("Пользователь не найден".into())
        }
    }

    pub(crate) fn withdraw_internal(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            if *balance >= amount {
                *balance -= amount;
                Ok(())
            } else {
                Err("Недостаточно средств".into())
            }
        } else {
            Err("Пользователь не найден".into())
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

