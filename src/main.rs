
mod bank;
use bank::Bank;
mod ledger;

fn main() {
    let mut bank = Bank::new(5);

    bank.deposit(0, 0, 0, 300);
    bank.withdraw(0, 1, 0, 200);
    bank.withdraw(0, 1, 0, 200);

    
    bank.deposit(0, 2, 1, 500);
    // 0 has 100, 1 has 500
    bank.print_account();
    bank.transfer(0, 3, 1, 0, 300);
    // 0 has 400, 1 has 200
    bank.print_account();
    bank.check_balance(1);

}