use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

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
pub fn InitBank(num_workers: i32, filename: &str){
    let mut bank = Bank::new(10);
    bank.print_account();
    let mut bank = Arc::new(Mutex::new(Bank::new(10)));

    let mut ledger_list = read_ledger_file(filename);

    let mut handles = vec![];
    let random_numbers = [751, 42, 879, 614, 187, 935, 263, 512, 933, 65];
    for i in 0..10 {
        let mut bank = Arc::clone(&bank);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(random_numbers[i]));
            bank.lock().unwrap().deposit(i as i32, 0, 0, 69);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let lock = bank.lock().unwrap();
    lock.print_account();
    println!("---------------------------------------------------------------------\nFINISHED INIT BANK"); 
}

