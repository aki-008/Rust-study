#![allow(unused)]

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const DATA_FILE: &str = "bank_data.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Account {
    name: String,
    balance: f64,
    history: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
struct Bank {
    accounts: HashMap<String, Account>,
}

impl Bank {
    fn load() -> Self {
        if Path::new(DATA_FILE).exists() {
            let data = fs::read_to_string(DATA_FILE).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Bank::default()
        }
    }

    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(DATA_FILE, json);
        }
    }

    fn next_account_number(&self) -> String {
        match self
            .accounts
            .keys()
            .filter_map(|k| k.parse::<u32>().ok())
            .max()
        {
            Some(max) => (max + 1).to_string(),
            None => "1001".to_string(),
        }
    }

    fn log(&mut self, acc_num: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        if let Some(acc) = self.accounts.get_mut(acc_num) {
            acc.history.push(format!("[{}] {}", timestamp, message));
        }
    }

    fn create_account(&mut self, name: &str, opening_balance: f64) -> String {
        let acc_num = self.next_account_number();
        self.accounts.insert(
            acc_num.clone(),
            Account {
                name: name.to_string(),
                balance: (opening_balance * 100.0).round() / 100.0,
                history: Vec::new(),
            },
        );
        self.log(
            &acc_num,
            &format!(
                "Account created with opening balance {:.2}",
                opening_balance
            ),
        );
        self.save();
        acc_num
    }

    fn deposit(&mut self, acc_num: &str, amount: f64) -> Result<f64, String> {
        if !self.accounts.contains_key(acc_num) {
            return Err("Accounts not found.".to_string());
        }
        if amount <= 0.0 {
            return Err("Deposit amount must be positive.".to_string());
        }
        let new_balance = {
            let acc = self.accounts.get_mut(acc_num).unwrap();
            acc.balance += amount;
            acc.balance
        };
        self.log(acc_num, &format!("Deposited {:?}", amount));
        self.save();
        Ok(new_balance)
    }
    fn withdraw(&mut self, acc_num: &str, amount: f64) -> Result<f64, String> {
        if !self.accounts.contains_key(acc_num) {
            return Err("Account not found.".to_string());
        }
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive.".to_string());
        }
        let acc = self.accounts.get_mut(acc_num).unwrap();
        if amount > acc.balance {
            return Err("Insufficent funds.".to_string());
        }
        acc.balance -= amount;
        let new_balance = acc.balance;
        self.log(acc_num, &format!("Withdrew {:.2}", amount));
        self.save();
        Ok(new_balance)
    }

    fn get_balance(&self, acc_num: &str) -> Option<f64> {
        self.accounts.get(acc_num).map(|a| a.balance)
    }

    fn get_history(&self, acc_num: &str) -> Option<&Vec<String>> {
        self.accounts.get(acc_num).map(|a| &a.history)
    }

    fn delete_account(&mut self, acc_num: &str) -> Result<(), String> {
        if self.accounts.remove(acc_num).is_none() {
            return Err("Account not found.".to_string());
        }
        self.save();
        Ok(())
    }
}

fn print_header(title: &str) {
    println!("\n{}", "=".repeat(40));
    println!("{:^40}", title);
    println!("{}", "=".repeat(40));
}

fn prompt(label: &str) -> String {
    print!("{}", label);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_f64(label: &str) -> Option<f64> {
    let raw = prompt(label);
    if raw.is_empty() {
        return Some(0.0);
    }
    raw.parse::<f64>().ok()
}

fn main() {
    let mut bank = Bank::load();

    let menu = "\n1. Create Account\n2. Deposit\n3. Withdraw\n4. Check Balance\n5. Transaction History\n6. List All Accounts\n7. Delete Account\n8. Exit\n";

    print_header("BANK CLI");

    loop {
        println!("{}", menu);
        let choice = prompt("Select an option (1-8): ");

        match choice.as_str() {
            "1" => {
                let name = prompt("Enter account holder name: ");
                let opening = match prompt_f64("Enter opening balance (0 for none): ") {
                    Some(v) => v,
                    None => {
                        println!("Invalid amount. Defaulting to 0.");
                        0.0
                    }
                };
                let acc_num = bank.create_account(&name, opening);
                println!("Account created successfully! Account Number: {}", acc_num);
            }
            "2" => {
                           let acc_num = prompt("Enter account number: ");
                           match prompt_f64("Enter deposit amount: ") {
                               Some(amount) => match bank.deposit(&acc_num, amount) {
                                   Ok(balance) => println!("Deposited {:.2}. New balance: {:.2}", amount, balance),
                                   Err(e) => println!("{}", e),
                               },
                               None => println!("Invalid amount."),
                           }
                       }

                       "3" => {
                           let acc_num = prompt("Enter account number: ");
                           match prompt_f64("Enter withdrawal amount: ") {
                               Some(amount) => match bank.withdraw(&acc_num, amount) {
                                   Ok(balance) => println!("Withdrew {:.2}. New balance: {:.2}", amount, balance),
                                   Err(e) => println!("{}", e),
                               },
                               None => println!("Invalid amount."),
                           }
                       }

                       "4" => {
                           let acc_num = prompt("Enter account number: ");
                           match bank.get_balance(&acc_num) {
                               Some(balance) => println!("Current balance: {:.2}", balance),
                               None => println!("Account not found."),
                           }
                       }

                       "5" => {
                           let acc_num = prompt("Enter account number: ");
                           match bank.get_history(&acc_num) {
                               Some(history) if history.is_empty() => println!("No transactions yet."),
                               Some(history) => {
                                   print_header(&format!("HISTORY - Account {}", acc_num));
                                   for entry in history {
                                       println!("{}", entry);
                                   }
                               }
                               None => println!("Account not found."),
                           }
                       }

                       "6" => {
                           if bank.accounts.is_empty() {
                               println!("No accounts found.");
                           } else {
                               print_header("ALL ACCOUNTS");
                               // HashMap doesn't guarantee iteration order, so we
                               // collect the keys into a Vec and sort it ourselves
                               // (numerically, since account numbers are digits)
                               // just so the listing looks tidy — this doesn't
                               // change how accounts are stored, only how we choose
                               // to walk through them for display.
                               let mut acc_nums: Vec<&String> = bank.accounts.keys().collect();
                               acc_nums.sort_by_key(|k| k.parse::<u32>().unwrap_or(0));

                               for acc_num in acc_nums {
                                   let info = &bank.accounts[acc_num];
                                   println!("{} | {:<20} | Balance: {:.2}", acc_num, info.name, info.balance);
                               }
                           }
                       }

                       "7" => {
                           let acc_num = prompt("Enter account number to delete: ");
                           let confirm = prompt(&format!(
                               "Are you sure you want to delete account {}? (y/n): ",
                               acc_num
                           ));
                           if confirm.to_lowercase() == "y" {
                               match bank.delete_account(&acc_num) {
                                   Ok(()) => println!("Account deleted."),
                                   Err(e) => println!("{}", e),
                               }
                           } else {
                               println!("Cancelled.");
                           }
                       }

                       "8" => {
                           println!("Thank you for using the Bank Management System. Goodbye!");
                           break;
                       }

                       _ => println!("Invalid option. Please choose between 1-8."),
        }
    }
}
