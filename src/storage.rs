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
    pub(crate) accounts: HashMap<Name, Balance>,
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

    pub fn get_all(&self) -> impl Iterator<Item = (Name, i64)> + '_ {
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
use std::io::{BufReader, BufWriter, Cursor, Write};

#[test]
fn test_load_data_existing_cursor() {
    // Создаём данные в памяти, как будто это CSV-файл
    let data = b"John,100\nAlice,200\nBob,50\n";
    let mut cursor = Cursor::new(&data[..]);

    // Читаем данные из Cursor
    let mut storage = Storage::new();
    let reader = BufReader::new(&mut cursor);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() == 2 {
            let name = parts[0].to_string();
            let balance: i64 = parts[1].parse().unwrap_or(0);
            UserManager::add_user(&mut storage, name.clone());
            BalanceManager::deposit(&mut storage, &name, balance).unwrap();
        }
    }

    assert_eq!(
        BalanceManager::get_balance(&storage, &"John".to_string()),
        Some(100)
    );
    assert_eq!(
        BalanceManager::get_balance(&storage, &"Alice".to_string()),
        Some(200)
    );
    assert_eq!(
        BalanceManager::get_balance(&storage, &"Bob".to_string()),
        Some(50)
    );
    assert_eq!(
        BalanceManager::get_balance(&storage, &"Vasya".to_string()),
        None
    ); // нет в данных
}

#[test]
fn test_save_writes_to_cursor_correctly() {
    // Создаём Storage и добавляем пользователей
    let mut storage = Storage::new();
    UserManager::add_user(&mut storage, "John".to_string());
    UserManager::add_user(&mut storage, "Alice".to_string());
    BalanceManager::deposit(&mut storage, &"John".to_string(), 150).unwrap();
    BalanceManager::deposit(&mut storage, &"Alice".to_string(), 300).unwrap();

    // Сохраняем в память через BufWriter
    let buffer = Vec::new();
    let mut cursor = Cursor::new(buffer);
    {
        let mut writer = BufWriter::new(&mut cursor);
        for (name, balance) in storage.get_all() {
            writeln!(writer, "{},{}", name, balance).unwrap();
        }
        writer.flush().unwrap();
    }

    // Читаем обратно из памяти
    cursor.set_position(0);
    let mut lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();
    lines.sort(); // сортируем для сравнения

    assert_eq!(lines, vec!["Alice,300", "John,150"]);
}
