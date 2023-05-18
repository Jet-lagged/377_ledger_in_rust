use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

use crate::bank::Bank;

pub enum Mode {
    Deposit,
    Withdraw,
    Transfer,
    CheckBalance,
}
/// The `Ledger` struct represents a financial transaction with information about the sender(`from`), receiver(`to`),
/// `amount`, `mode`, and `ledger_id`.
pub struct Ledger {
    pub from: i32,
	pub to: i32,
    pub amount: i32,
    pub mode: Mode,
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
                match l.mode {
                    Mode::Deposit => bank_lock.deposit(worker_id, l.ledger_id, l.from, l.amount),
                    Mode::Withdraw => bank_lock.withdraw(worker_id, l.ledger_id, l.from, l.amount),
                    Mode::Transfer => bank_lock.transfer(worker_id, l.ledger_id, l.from as usize, l.to as usize, l.amount),
                    Mode::CheckBalance => bank_lock.check_balance(worker_id, l.ledger_id, l.from),
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

