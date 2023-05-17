
use std::sync::{Mutex, Arc};
use std::thread;

mod bank;
use bank::Bank;
mod ledger;

fn main() {
    ledger::InitBank(0, "src\\pressure_test.txt");
}