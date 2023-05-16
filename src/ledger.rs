
mod bank;
use bank::Bank;

enum Mode {
    Deposit,
    Withdraw,
    Transfer,
    Check_Balance,
}
struct Ledger {
	from: i32,
	to: i32,
	amount: i32,
  	mode: Mode,
	ledgerID: i32,
}