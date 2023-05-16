use std::sync::{Arc, Condvar, Mutex};
use std::io::{self, BufRead, BufReader};
use std::fs::File;
mod bank;
use bank::Bank;

pub enum Mode {
    Deposit,
    Withdraw,
    Transfer,
    Check_Balance,
}
pub struct Ledger {
	pub from: i32,
	pub to: i32,
	pub amount: i32,
	pub mode: Mode,
	pub ledgerID: i32,
}
pub struct BoundedBuffer {
    buffer: Vec<Ledger>,
    max_size: usize,
    count: usize,
    lock: Mutex<()>,
    not_full: Condvar,
    not_empty: Condvar,
}

impl BoundedBuffer {
    pub fn new(max_size: usize) -> Self {
        BoundedBuffer {
            buffer: Vec::<Ledger>::with_capacity(max_size),
            max_size,
            count: 0,
            lock: Mutex::new(()),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

	pub fn put(&mut self, item: Ledger) {
        let mut lock = self.lock.lock().unwrap();
        while self.count == self.max_size {
            lock = self.not_full.wait(lock).unwrap();
        }
        self.buffer.push = item;
        self.count += 1;
        self.not_empty.notify_one();
    }

	pub fn get(&mut self) -> Ledger {
        let mut lock = self.lock.lock().unwrap();
        while self.count == 0 {
            lock = self.not_empty.wait(lock).unwrap();
        }
        let item = self.buffer.remove(0).clone();
        self.count -= 1;
        self.not_full.notify_one();
        item
    }
}

pub fn read_ledger_file(filename: &str, bb: BoundedBuffer) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split_whitespace().collect();

		if fields.len() != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid number of fields in ledger entry",
            ));
        }

        let from_id = fields[0].parse::<i32>().unwrap();
        let to_id = fields[1].parse::<i32>().unwrap();
        let amount = fields[2].parse::<i32>().unwrap();
        let mode = match values[3] {
            "D" => Mode::Deposit,
            "W" => Mode::Withdraw,
            "T" => Mode::Transfer,
            "C" => Mode::CheckBalance,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid mode in ledger entry",
                ))
            }
        };
        let ledger = Ledger {
            from_id,
            to_id,
            amount,
            mode,
        };
        bb.lock().unwrap().put(ledger);
    }
}