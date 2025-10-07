use crate::storage::{Name, Storage};

pub struct UserManager;

impl UserManager {
    /// Adds a new user with zero balance
    /// Returns Some(0) if user was created, None if user already exists
    pub fn add_user(storage: &mut Storage, name: Name) -> Option<i64> {
        storage.add_user_internal(name)
    }

    /// Removes a user and returns their final balance
    /// Returns Some(balance) if user existed, None if user not found
    pub fn remove_user(storage: &mut Storage, name: &Name) -> Option<i64> {
        storage.remove_user_internal(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut storage = Storage::new();
        assert_eq!(UserManager::add_user(&mut storage, "Alice".to_string()), Some(0));
        assert_eq!(UserManager::add_user(&mut storage, "Alice".to_string()), None);
    }

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();
        UserManager::add_user(&mut storage, "Bob".to_string());
        
        assert_eq!(UserManager::remove_user(&mut storage, &"Bob".to_string()), Some(0));
        assert_eq!(UserManager::remove_user(&mut storage, &"Bob".to_string()), None);
    }
}

