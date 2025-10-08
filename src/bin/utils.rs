use std::io::{self, BufRead, Write};

use bank_system::{
    balance_manager::BalanceManager,
    storage::{Name, Storage},
    transaction::{Deposit, Transaction, Transfer},
    user_manager::UserManager,
};

const FILE_NAME: &str = "balance.csv";

fn main() {
    let mut storage = Storage::load_data("balance.csv");

    println!("=== Bank CLI Utils ===");
    println!("Команды:");
    println!("  add <name> <balance>      - добавить пользователя");
    println!("  remove <name>             - удалить пользователя");
    println!("  list                      - показать всех пользователей");
    println!("  deposit <name> <amount>   - пополнить баланс");
    println!("  withdraw <name> <amount>  - снять со счёта");
    println!("  balance <name>            - показать баланс");
    println!("  transfer <from> <to> <amount> - перевести деньги");
    println!("  exit                      - выйти");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap(); // показываем приглашение

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break; // EOF
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "add" => {
                if args.len() != 3 {
                    println!("Пример: add John 100");
                    continue;
                }
                let name: Name = args[1].to_string();
                let balance: i64 = match args[2].parse() {
                    Ok(b) => b,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };
                if UserManager::add_user(&mut storage, name.clone()).is_some() {
                    let _ = BalanceManager::deposit(&mut storage, &name, balance);
                    println!("Пользователь {} добавлен с балансом {}", name, balance);
                    storage.save(FILE_NAME);
                } else {
                    println!("Пользователь {} уже существует", name);
                }
            }
            "remove" => {
                if args.len() != 2 {
                    println!("Пример: remove John");
                    continue;
                }
                let name = args[1];
                if UserManager::remove_user(&mut storage, &name.to_string()).is_some() {
                    println!("Пользователь {} удалён", name);
                    storage.save(FILE_NAME);
                } else {
                    println!("Пользователь {} не найден", name);
                }
            }
            "list" => {
                if args.len() != 1 {
                    println!("Пример: list");
                    continue;
                }
                for (name, balance) in storage.get_all() {
                    println!("{}: {}", name, balance);
                }
            }
            "deposit" => {
                if args.len() != 3 {
                    println!("Пример: deposit John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                let tx = Deposit {
                    account: name.clone(),
                    amount,
                };
                // Применяем транзакцию
                match tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: депозит {} на {}", name, amount);
                        storage.save(FILE_NAME);
                    }
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
            }
            "withdraw" => {
                if args.len() != 3 {
                    println!("Пример: withdraw John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };
                match BalanceManager::withdraw(&mut storage, &name, amount) {
                    Ok(_) => {
                        println!("С баланса пользователя {} снято {}", name, amount);
                        storage.save(FILE_NAME);
                    }
                    Err(e) => println!("Ошибка: {}", e),
                }
            }
            "balance" => {
                if args.len() != 2 {
                    println!("Пример: balance John");
                    continue;
                }
                let name = args[1].to_string();
                match BalanceManager::get_balance(&storage, &name) {
                    Some(b) => println!("Баланс пользователя {}: {}", name, b),
                    None => println!("Пользователь {} не найден", name),
                }
            }
            "transfer" => {
                if args.len() != 4 {
                    println!("Пример: tx_transfer Alice Bob 50");
                    continue;
                }
                let from = args[1].to_string();
                let to = args[2].to_string();
                let amount: i64 = match args[3].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                let tx = Transfer {
                    from: from.clone(),
                    to: to.clone(),
                    amount,
                };
                match tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: перевод {} на {}", from, to);
                        storage.save(FILE_NAME);
                    }
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
            }
            "exit" => break,
            _ => println!("Неизвестная команда"),
        }
    }

    println!("Выход из CLI, все изменения сохранены.");
}
