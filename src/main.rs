
use std::sync::{Mutex, Arc};

mod bank;
use bank::Bank;
mod ledger;

fn main() {
    let mut bank = Bank::new(5);


    let line = "0 0 500 D";
    let mut buffer: ledger::BoundedBuffer = ledger::BoundedBuffer::new(5);
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    ledger::read_ledger_line(line, &mut buffer, counter);
    println!("{:#?}", buffer);
}