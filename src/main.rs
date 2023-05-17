
use std::sync::{Mutex, Arc};

mod bank;
use bank::Bank;
mod ledger;

fn main() {
    let mut bank = Bank::new(5);
    ledger::InitBank(0, "src\\pressure_test.txt");
}