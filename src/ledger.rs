use std::sync::{Arc, Condvar, Mutex};

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
pub fn read_ledger_line(line: &str, bb: &mut BoundedBuffer, ledger_counter: Arc<Mutex<i32>>) {
    let fields: Vec<&str> = line.split_whitespace().collect();
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
