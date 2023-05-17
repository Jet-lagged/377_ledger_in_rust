



mod bank;
use bank::Bank;
mod ledger;

fn main() {
    ledger::init_bank(5, "src\\ledger_test.txt");
}