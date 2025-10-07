use std::{
    collections::{HashMap, hash_map},
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use crate::{balance_manager::BalanceManager, user_manager::UserManager};

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

    fn get_all(&self) -> impl Iterator<Item = (Name, i64)> + '_ {
        self.accounts.iter().map(|(n, b)| (n.clone(), *b))
    }

    /// Загружает данные из CSV-файла или создаёт хранилище с дефолтными пользователями
    pub fn load_data(file: &str) -> Storage {
        let mut storage = Storage::new();

        // Проверяем, существует ли файл
        if Path::new(file).exists() {
            // Открываем файл
            let file = File::open(file).unwrap();

            // Оборачиваем файл в BufReader
            // BufReader читает данные блоками и хранит их в буфере,
            // поэтому построчное чтение (lines()) работает быстрее, чем читать по байту
            let reader = io::BufReader::new(file);

            // Читаем файл построчно
            for line in reader.lines().map_while(Result::ok) {
                // Разделяем строку по запятой: "Name,Balance"
                let parts: Vec<&str> = line.trim().split(',').collect();

                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    // Пробуем преобразовать баланс из строки в число
                    let balance: i64 = parts[1].parse().unwrap_or(0);

                    // Добавляем пользователя и выставляем баланс
                    UserManager::add_user(&mut storage, name.clone());
                    let _ = BalanceManager::deposit(&mut storage, &name, balance);
                }
            }
        } else {
            // если файла нет, создаём пользователей с нуля
            for u in ["John", "Alice", "Bob", "Vasya"] {
                UserManager::add_user(&mut storage, u.to_string());
            }
        }

        storage
    }

    /// Сохраняет текущее состояние Storage в CSV-файл
    pub fn save(&self, file: &str) {
        let mut data = String::new();

        // Собираем все данные в одну строку формата "Name,Balance"
        for (name, balance) in self.get_all() {
            data.push_str(&format!("{},{}\n", name, balance));
        }

        // Записываем в файл
        // Здесь мы не используем BufWriter, потому что сразу пишем всю строку целиком.
        fs::write(file, data).expect("Не удалось записать файл");
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_load_data_existing_file() {
        let file_path = "test_load.csv";

        // Создаём файл с исходными данными
        let mut f = File::create(file_path).unwrap();
        writeln!(f, "John,100").unwrap();
        writeln!(f, "Alice,200").unwrap();
        writeln!(f, "Bob,50").unwrap();

        // Загружаем Storage
        let storage = Storage::load_data(file_path);

        assert_eq!(BalanceManager::get_balance(&storage, &"John".to_string()), Some(100));
        assert_eq!(BalanceManager::get_balance(&storage, &"Alice".to_string()), Some(200));
        assert_eq!(BalanceManager::get_balance(&storage, &"Bob".to_string()), Some(50));
        // Пользователь Vasya не добавлен в файле, поэтому None
        assert_eq!(BalanceManager::get_balance(&storage, &"Vasya".to_string()), None);

        // Удаляем тестовый файл
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_save_creates_file_with_correct_data() {
        let file_path = "test_save.csv";

        // Создаём Storage и добавляем пользователей
        let mut storage = Storage::new();
        UserManager::add_user(&mut storage, "John".to_string());
        UserManager::add_user(&mut storage, "Alice".to_string());
        BalanceManager::deposit(&mut storage, &"John".to_string(), 150).unwrap();
        BalanceManager::deposit(&mut storage, &"Alice".to_string(), 300).unwrap();

        // Сохраняем в файл
        storage.save(file_path);

        // Читаем файл обратно и проверяем содержимое
        let contents = fs::read_to_string(file_path).unwrap();
        let mut lines: Vec<&str> = contents.lines().collect();
        lines.sort(); // сортируем, так как get_all() может возвращать в любом порядке

        assert_eq!(lines, vec!["Alice,300", "John,150"]);

        // Удаляем тестовый файл
        fs::remove_file(file_path).unwrap();
    }
}
