use std::sync::{Arc, Condvar, Mutex};
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
#[derive(Debug)]
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
        self.buffer.push(item);
        self.count += 1;
        drop(lock);
        self.not_empty.notify_one();
    }

	pub fn get(&mut self) -> Ledger {
        let mut lock = self.lock.lock().unwrap();
        while self.count == 0 {
            lock = self.not_empty.wait(lock).unwrap();
        }
        let item = self.buffer.remove(0);
        self.count -= 1;
        drop(lock);
        self.not_full.notify_one();
        item
    }
}


// not actually implemented
pub fn read_ledger_line(line: String, bb: &mut BoundedBuffer, ledger_counter: Arc<Mutex<i32>>) {
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
            return;
        }
    };
    let mut counter_lock = ledger_counter.lock().unwrap();
    *counter_lock += 1;
    let counter = *counter_lock;
    let ledger = Ledger {
        from,
        to,
        amount,
        mode,
        ledger_id: counter,
    };

    // write to buffer
    bb.put(ledger);
    drop(counter_lock);
}

pub fn InitBank(num_workers: i32, filename: &str){
    let mut bank = Bank::new(10);
    bank.print_account();

    let file = File::open(filename).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let num_threads = 3;
    let lines_per_thread = lines.len() / num_threads;
    let remaining_lines = lines.len() % num_threads;

    // Create the reader threads
    let mut thread_handles = Vec::new();
    for i in 0..num_threads {
        let additional_line = if i < remaining_lines { 1 } else { 0 };
        let lines_slice = lines.split_off(lines.len() - (lines_per_thread + additional_line));
        // TESTING CODE TO SEE HOW MUCH EACH READING THREAD HANDLES
        // println!("thread 1 handles {} lines", lines_per_thread + additional_line);

        let handle = thread::spawn(move || {
            // Process the assigned lines
            for line in lines_slice {
                // Process the line
                println!("Thread {}: {}", i, line);
            }
        });

        thread_handles.push(handle);
    }

    // Wait for all reader threads to finish
    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("---------------------------------------------------------------------\nFINISHED INIT BANK");
}