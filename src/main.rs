mod bank;
mod ledger;
use std::env;
use ledger::init_bank;

/// Takes command line arguments, checks if they are valid, and initializes a bank
/// with the specified number of threads and ledger file.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: final_project <num_of_threads> <ledger_file> <sleep: bool>");
    } else {
        let num_threads: i32 = match &args[1].parse() {
            Ok(num) => *num,
            Err(_) => {
                println!("num_threads should be a number");
                return;
            }
        };
        let ledger_file = format!("ledgers/{}", &args[2]);
        let ledger_file = ledger_file.as_str();

        let sleep = match &args[3].parse::<bool>() {
            Ok(value) => *value,
            Err(_) => {
                println!("sleep should be a boolean.");
                return;
            }
        };
        init_bank(num_threads, ledger_file, sleep);
    }
}