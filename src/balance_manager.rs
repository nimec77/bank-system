use crate::storage::{Name, Storage};

pub struct BalanceManager;

impl BalanceManager {
    /// Gets the balance of a user
    /// Returns Some(balance) if user exists, None otherwise
    pub fn get_balance(storage: &Storage, name: &Name) -> Option<i64> {
        storage.get_balance_internal(name)
    }

    /// Deposits amount into user's account
    /// Returns Ok(()) if successful, Err if user not found
    pub fn deposit(storage: &mut Storage, name: &Name, amount: i64) -> Result<(), String> {
        storage.deposit_internal(name, amount)
    }

    /// Withdraws amount from user's account
    /// Returns Ok(()) if successful, Err if user not found or insufficient funds
    pub fn withdraw(storage: &mut Storage, name: &Name, amount: i64) -> Result<(), String> {
        storage.withdraw_internal(name, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_manager::UserManager;

    #[test]
    fn test_get_balance() {
        let mut storage = Storage::new();
        UserManager::add_user(&mut storage, "Alice".to_string());

        assert_eq!(
            BalanceManager::get_balance(&storage, &"Alice".to_string()),
            Some(0)
        );
        assert_eq!(
            BalanceManager::get_balance(&storage, &"Nobody".to_string()),
            None
        );
    }

    #[test]
    fn test_deposit_and_withdraw() {
        let mut storage = Storage::new();
        UserManager::add_user(&mut storage, "Charlie".to_string());

        // Пополнение
        assert!(BalanceManager::deposit(&mut storage, &"Charlie".to_string(), 200).is_ok());
        assert_eq!(
            BalanceManager::get_balance(&storage, &"Charlie".to_string()),
            Some(200)
        );

        // Успешное снятие
        assert!(BalanceManager::withdraw(&mut storage, &"Charlie".to_string(), 150).is_ok());
        assert_eq!(
            BalanceManager::get_balance(&storage, &"Charlie".to_string()),
            Some(50)
        );

        // Ошибка: недостаточно средств
        assert!(BalanceManager::withdraw(&mut storage, &"Charlie".to_string(), 100).is_err());
        assert_eq!(
            BalanceManager::get_balance(&storage, &"Charlie".to_string()),
            Some(50)
        );
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();

        // Депозит несуществующему пользователю
        assert!(BalanceManager::deposit(&mut storage, &"Dana".to_string(), 100).is_err());

        // Снятие у несуществующего пользователя
        assert!(BalanceManager::withdraw(&mut storage, &"Dana".to_string(), 50).is_err());

        // Баланс у несуществующего пользователя
        assert_eq!(
            BalanceManager::get_balance(&storage, &"Dana".to_string()),
            None
        );
    }
}
