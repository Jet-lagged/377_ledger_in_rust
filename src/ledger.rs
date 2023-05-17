use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;


use crate::bank::Bank;

#[derive(Debug)]
pub enum Mode {
    Deposit,
    Withdraw,
    Transfer,
    CheckBalance,
}
#[derive(Debug)]
pub struct Ledger {
	pub from: i32,
	pub to: i32,
	pub amount: i32,
	pub mode: Mode,
	pub ledger_id: i32,
}

// not actually implemented
pub fn read_ledger_file(filename: &str) -> Vec<Ledger> {
    let file = File::open(filename).unwrap();
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


// SOMEHOW
// this mess of a InitBank works and final balance of account 0 is 200
pub fn init_bank(num_workers: i32, filename: &str){
    let bank = Bank::new(12);
    bank.print_account();
    let bank = Arc::new(Mutex::new(Bank::new(10)));

    let ledger_list = Arc::new(Mutex::new(read_ledger_file(filename)));

    let mut handles = vec![];
    for i in 0..num_workers as usize {
        let bank = Arc::clone(&bank);
        let ledger_list = Arc::clone(&ledger_list);
        let handle = thread::spawn(move || {
            loop {
                let worker_id = i as i32;
                let mut ledger_lock = ledger_list.lock().unwrap();
                if ledger_lock.is_empty() {
                    break; // lock dropped here
                } 

                let ledger:Ledger = ledger_lock.remove(0);
                drop(ledger_lock);

                let mut bank_lock = bank.lock().unwrap();
                match ledger.mode {
                    Mode::Deposit => bank_lock.deposit(worker_id, ledger.ledger_id, ledger.from, ledger.amount),
                    Mode::Withdraw => bank_lock.withdraw(worker_id, ledger.ledger_id, ledger.from, ledger.amount),
                    Mode::Transfer => bank_lock.transfer(worker_id, ledger.ledger_id, ledger.from as usize, ledger.to as usize, ledger.amount),
                    Mode::CheckBalance => bank_lock.check_balance(worker_id, ledger.from),
                }
                drop(bank_lock);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let lock = bank.lock().unwrap();
    lock.print_account();
    drop(lock);
    println!("---------------------------------------------------------------------\nFINISHED INIT BANK"); 
}

