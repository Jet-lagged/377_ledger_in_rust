use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

use crate::bank::Bank;

pub enum Mode {
    /// `Deposit` is a mode of a financial transaction represented by the `Ledger` struct. It adds a
    /// specified amount of currency to the account of the sender (`from` property of the `Ledger`
    /// struct).
    Deposit,
    /// `Withdraw` is a mode of a financial transaction represented by the `Ledger` struct. It subtracts
    /// a specified amount of currency from the account of the sender (`from` property of the `Ledger`
    /// struct).
    Withdraw,
    /// `Transfer` is a mode of a financial transaction represented by the `Ledger` struct. It transfers
    /// a specified amount of currency from the account of the sender (`from` property of the `Ledger`
    /// struct) to the account of the recipient (`to` property of the `Ledger` struct).
    Transfer,
    /// `CheckBalance` is a mode of a financial transaction represented by the `Ledger` struct. It is
    /// used to check the balance of a specific account (`from` property of the `Ledger` struct) in the
    /// bank. It does not add or subtract any currency from the account.
    CheckBalance,
}
/// The `Ledger` struct represents a financial transaction with information about the sender(`from`), receiver(`to`),
/// `amount`, `mode`, and `ledger_id`.
pub struct Ledger {
    /// This property represents the account number or identifier of the sender of a transaction in a
    /// ledger. 
    pub from: i32,
    /// This property represents the account number or identifier of the recipient of a transaction in a
    /// ledger. Used for transfering money: the `to` account gains money.
	pub to: i32,
	/// This property represents the amount of currency being handled in the transaction
    pub amount: i32,
	/// This property represents the `Mode of` the transaction, which can be one of four options: `Deposit`,
    /// `Withdraw`, `Transfer`, or `CheckBalance`.
    pub mode: Mode,
	/// This property represents the unique identifier of a ledger transaction.
    pub ledger_id: i32,
}


/// Reads a ledger file and returns a vector of Ledger structs.
/// 
/// Arguments:
/// 
/// * `filename`: A string representing the name of the ledger file to be read.
/// 
/// Returns:
/// 
/// The function `read_ledger_file` returns a vector of `Ledger` structs.
pub fn read_ledger_file(filename: &str) -> Vec<Ledger> {
    let file = File::open(filename).expect("Usage: final_project <num_of_threads> <ledger_file>\nError: ledger_file invalid");
    let reader = BufReader::new(file);

    let mut ledger_entries = Vec::new();
    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result.unwrap();
        let fields: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let from = fields[0].parse::<i32>().unwrap();
        let to = fields[1].parse::<i32>().unwrap();
        let amount = fields[2].parse::<i32>().unwrap();
        let mode = match fields[3] {
            "D" => Mode::Deposit,
            "W" => Mode::Withdraw,
            "T" => Mode::Transfer,
            "C" => Mode::CheckBalance,
            _ => {
                println!("Error: mod not specified");
                return Vec::new();
            }
        };
        let ledger = Ledger {
            from,
            to,
            amount,
            mode,
            ledger_id: i as i32,
        };
        ledger_entries.push(ledger);
    }
    return ledger_entries;
}



/// Initializes a bank with a given number of workers and processes ledger transactions in
/// a multi-threaded manner. Prints account states before and after all ledgers.
/// 
/// Arguments:
/// 
/// * `num_workers`: The number of worker threads to create for processing ledger transactions.
/// * `filename`: The name of the ledger file that contains the transactions to be executed by the bank.
/// * `sleep`: Determines if the thread sleeps right before processing the ledgers: used to show multi-threading more clearly
pub fn init_bank(num_workers: i32, filename: &str, sleep: bool){
    let bank = Bank::new(10);
    bank.print_account();
    let bank = Arc::new(Mutex::new(Bank::new(10)));
    let ledger_list = Arc::new(Mutex::new(read_ledger_file(filename)));

    // threads created here
    let mut handles = vec![];
    for i in 0..num_workers as usize {
        let bank = Arc::clone(&bank);
        let ledger_list = Arc::clone(&ledger_list);
        let handle = thread::spawn(move || {
            loop {

                // read from ledger until list is empty
                let worker_id = i as i32;
                let mut ledger_lock = ledger_list.lock().unwrap();
                if ledger_lock.is_empty() {
                    break; // lock dropped here
                } 
                let l:Ledger = ledger_lock.remove(0);
                drop(ledger_lock);
                
                // Sleep not neccessary, used to showcase multi-threaded nature more clearly
                if sleep {
                    if i == 3 || i == 6 || i == 7 {
                        thread::sleep(Duration::from_millis(300 as u64));
                    } else if i == 2 || i == 9 || i == 8 {
                        thread::sleep(Duration::from_millis(50 as u64));
                    } else {
                        thread::sleep(Duration::from_millis(150 as u64));
                    }
                }

                // process ledger
                let mut bank_lock = bank.lock().unwrap();
                // uncomment to show that only one thread can access the bank at a time
                // thread::sleep(Duration::from_millis(3000 as u64));
                if l.from < 0 || l.from >= bank_lock.num {
                    println!("Worker {:2} -FAILED-  ledger {:2}:   Ledger Error: no account of this ID", worker_id, l.ledger_id);
                } else {
                    match l.mode {
                        Mode::Deposit => bank_lock.deposit(worker_id, l.ledger_id, l.from, l.amount),
                        Mode::Withdraw => bank_lock.withdraw(worker_id, l.ledger_id, l.from, l.amount),
                        Mode::Transfer => bank_lock.transfer(worker_id, l.ledger_id, l.from as usize, l.to as usize, l.amount),
                        Mode::CheckBalance => bank_lock.check_balance(worker_id, l.ledger_id, l.from),
                    }
                }
                drop(bank_lock);
            } // thread loop ends
        }); // handle ends
        handles.push(handle);
    } // thread creation ends

    for handle in handles {
        handle.join().unwrap();
    }

    let lock = bank.lock().unwrap();
    lock.print_account();
    drop(lock);
}

