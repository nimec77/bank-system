use bank_system::balance_manager::BalanceManager;
use bank_system::storage::{Name, Storage};
use bank_system::user_manager::UserManager;
use std::env;

fn main() {
    let mut storage = Storage::new();

    // заранее добавим пачку пользователей
    let users = ["John", "Alice", "Bob", "Vasya"];
    for u in users.iter() {
        UserManager::add_user(&mut storage, u.to_string());
    }

    // собираем аргументы
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Использование:");
        eprintln!("  deposit <name> <amount>");
        eprintln!("  withdraw <name> <amount>");
        eprintln!("  balance <name>");
        return;
    }

    match args[1].as_str() {
        "deposit" => {
            if args.len() != 4 {
                eprintln!("Пример: deposit John 200");
                return;
            }
            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Сумма должна быть числом");
            match BalanceManager::deposit(&mut storage, &name, amount) {
                Ok(_) => println!("Пополнено: {} на {}", name, amount),
                Err(e) => println!("Ошибка: {}", e),
            }
        }
        "withdraw" => {
            if args.len() != 4 {
                eprintln!("Пример: withdraw John 100");
                return;
            }
            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Сумма должна быть числом");
            match BalanceManager::withdraw(&mut storage, &name, amount) {
                Ok(_) => println!("Снято: {} на {}", name, amount),
                Err(e) => println!("Ошибка: {}", e),
            }
        }
        "balance" => {
            if args.len() != 3 {
                eprintln!("Пример: balance John");
                return;
            }
            let name: Name = args[2].clone();
            match BalanceManager::get_balance(&storage, &name) {
                Some(b) => println!("Баланс {}: {}", name, b),
                None => println!("Пользователь {} не найден", name),
            }
        }
        _ => {
            eprintln!("Неизвестная команда");
        }
    }
}
