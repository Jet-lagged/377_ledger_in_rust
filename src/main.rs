
mod bank;
use bank::Bank;

fn main() {
    let mut bank = Bank::new(5);

    bank.deposit(0, 0, 0, 300);
    bank.print_account();
    bank.withdraw(0, 1, 0, 200);
    bank.print_account();
    bank.withdraw(0, 1, 0, 200);
    bank.print_account();
}