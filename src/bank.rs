use std::sync::{Mutex, Arc, RwLock};
#[derive(Debug)]
/// The `Bank` struct contains a number, a vector of `Account` structs, and two `Arc<Mutex<i32>>`
/// variables for tracking successful and failed operations.
/// 
/// Properties:
/// 
/// * `_num`: a private field of type `i32` in the `Bank` struct. It is used to indicate 
/// how many accounts are created.
/// * `accounts`: a vector of `Account` structs that belong to the `Bank`. It stores all
/// the accounts that the bank manages.
/// * `num_succ`: a shared counter that keeps track of the number of successful operations
/// performed by the bank. It is an `Arc` (atomic reference counting) wrapped `Mutex` that allows
/// multiple threads to safely access and modify the counter.
/// * `num_fail`: a shared counter that keeps track of the number of failed operations
/// performed by the bank. It is an `Arc` (atomic reference counting) wrapped `Mutex` that allows
/// multiple threads to safely access and modify the counter.
pub(crate) struct Bank {
    _num: i32,
    accounts: Vec<Account>,
    num_succ: Arc<Mutex<i32>>,
    num_fail: Arc<Mutex<i32>>,
}

#[derive(Debug)]
/// The `Account` struct represents a bank account with an account ID, balance, and a read-write lock.
/// 
/// Properties:
/// 
/// * `account_id`: An integer representing the unique identifier of an account.
/// * `balance`: The `balance` property is an integer that represents the amount of money in the
/// account. It could be positive, negative, or zero depending on whether the account has a surplus, a
/// deficit, or no balance at all.
/// * `lock`: The `lock` property is a `RwLock` which is a type of synchronization primitive in Rust
/// that allows multiple readers or a single writer to access a shared resource at the same time. In
/// this case, the `lock` is used to ensure that only one thread can access the `balance`
struct Account {
    account_id: i32,
    balance: i32,
    lock: RwLock<()>,
}

// TODO: MAKE A NOTE ABOUT THIS SOMEHOW
// say that it is currently hard coded that the code formats account 
// and ledger ids with min 2 char and money stuff with min 8 char
impl Bank {
    /// This function creates a new Bank instance with a specified number of accounts, each with an
    /// account ID incrementing from 0, a balance of 0, and a lock
    /// 
    /// Arguments:
    /// 
    /// * `_num`: The number of accounts to create in the bank.
    /// 
    /// Returns:
    /// 
    /// A new instance of the `Bank` struct is being returned.
    pub fn new(_num: i32) -> Self {
        let mut accounts = Vec::with_capacity(_num as usize);
        for i in 0.._num {
            accounts.push(Account {
                account_id: i,
                balance: 0,
                lock: RwLock::new(()),
            });
        }
        let num_succ = Arc::new(Mutex::new(0 as i32));
        let num_fail = Arc::new(Mutex::new(0 as i32));
        Bank {
            _num,
            num_succ,
            num_fail,
            accounts,
        }
    }

    /// Prints the account IDs and balances of all accounts in a bank, as well as
    /// the number of successful and failed transactions.
    pub fn print_account(&self) {
        for account in &self.accounts {
            let _lock = account.lock.read().unwrap();
            println!("ID# {:2} | {:9}", account.account_id, account.balance);
        } // auto drops lock

        let num_succ = self.num_succ.lock().unwrap();
        let num_fail = self.num_fail.lock().unwrap();
        println!("Success: {:2} Fails: {:2}", *num_succ, *num_fail);
    }

    /// Adds a specified amount to a specified account's balance and prints a success
    /// message. 
    /// 
    /// Arguments:
    /// 
    /// * `worker_id`: An integer representing the ID of the worker who is making the deposit.
    /// * `ledger_id`: The ID of the ledger where the deposit transaction is being recorded.
    /// * `account_id`: The `account_id` of the `Account` to which the deposit is being made.
    /// * `amount`: The amount of money to be deposited into the specified  `Account`.
    pub fn deposit(&mut self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) {
        // Success (always)
        let mut account = &mut self.accounts[account_id as usize];
        let account_lock = account.lock.write().unwrap();
        account.balance += amount;
        let message = format!(
            "Worker {:2} completed ledger {:2}: deposit {:9} into account {:2}",
            worker_id, ledger_id, amount, account_id
        );
        drop(account_lock);
        let mut num_succ = self.num_succ.lock().unwrap();
        *num_succ += 1;
        println!("{}", message);
        return;
    }

    /// This function allows a worker to withdraw a specified amount from a specified account, updating
    /// the account balance and logging the transaction.
    /// 
    /// Arguments:
    /// 
    /// * `worker_id`: An integer representing the ID of the worker performing the withdrawal operation.
    /// * `ledger_id`: The ID of the ledger from which the withdrawal is being made.
    /// * `account_id`: The `account_id` of the `Account` from which the withdrawal is being made.
    /// * `amount`: The amount of money to withdraw from the `Account`.
    pub fn withdraw(&mut self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) {
        let mut account = &mut self.accounts[account_id as usize];
        let account_lock = account.lock.write().unwrap();

        // Fail 
        if account.balance < amount {
            let message = format!(
                "Worker {:2} failed to complete ledger {:2}: withdraw {:9} from account {:2}",
                worker_id, ledger_id, amount, account_id
            );
            drop(account_lock);
            let mut num_fail = self.num_fail.lock().unwrap();
            *num_fail += 1;
            println!("{}", message);
            return;
        }

        // Success
        account.balance -= amount;
        let message = format!(
            "Worker {:2} completed ledger {:2}: withdraw {:9} from account {:2}",
            worker_id, ledger_id, amount, account_id
        );
        drop(account_lock);
        let mut num_succ = self.num_succ.lock().unwrap();
        *num_succ += 1;
        println!("{}", message);
        return;
    }

    /// The function transfers a specified amount of money from one account to another, with appropriate
    /// error handling and locking mechanisms to prevent deadlocks.
    /// 
    /// Arguments:
    /// 
    /// * `worker_id`: An integer representing the ID of the worker performing the transfer.
    /// * `ledger_id`: The ID of the ledger from which the transfer is being made.
    /// * `src_id`: The `account_id` of the source `Account` from which the money is being transferred.
    /// * `dest_id`: The `account_id` of the `Account` where the transferred amount will be deposited.
    /// * `amount`: The amount of money to be transferred from one `Account` to another.
    pub fn transfer(&mut self, worker_id: i32, ledger_id: i32, src_id: usize, dest_id: usize, amount: i32) {
        // Handle tranfering money to oneself
        if src_id == dest_id {
            let message = format!(
                "Worker {:2} failed to complete ledger {:2}: tranfer {:9} from account {:2} to account {:2}",
                worker_id, ledger_id, amount, src_id, dest_id
            );
            let mut num_fail = self.num_fail.lock().unwrap();
            *num_fail += 1;
            println!("{}", message);
            return;
        }
        
        // split ownership of the account vector to access two accounts at once
        let accounts = &mut self.accounts;
        let (left, right) = accounts.split_at_mut(dest_id.max(src_id));
        let (src_acnt, dest_acnt) = if src_id < dest_id {
            (&mut left[src_id], &mut right[dest_id - (dest_id.max(src_id))])
        } else {
            (&mut right[src_id - (dest_id.max(src_id))], &mut left[dest_id])
        };
        
        // assign locks in consistant order to prevent deadlock
        let (src_lock, dest_lock);
        if src_id < dest_id {
            src_lock = src_acnt.lock.write().unwrap();
            dest_lock = dest_acnt.lock.write().unwrap();
        } else {
            dest_lock = dest_acnt.lock.write().unwrap();
            src_lock = src_acnt.lock.write().unwrap();
        }
        
        // Fail
        if src_acnt.balance < amount {
            let message = format!(
                "Worker {:2} failed to complete ledger {:2}: tranfer {:9} from account {:2} to account {:2}",
                worker_id, ledger_id, amount, src_id, dest_id
            );
            drop(src_lock);
            drop(dest_lock);
            let mut num_fail = self.num_fail.lock().unwrap();
            *num_fail += 1;
            println!("{}", message);
            return;
        }

        // Success
        src_acnt.balance -= amount;
        dest_acnt.balance += amount;
        let message = format!(
            "Worker {:2} completed ledger {:2}: tranfer {:9} from account {:2} to account {:2}",
            worker_id, ledger_id, amount, src_id, dest_id
        );
        drop(src_lock);
        drop(dest_lock);
        let mut num_fail = self.num_fail.lock().unwrap();
        *num_fail += 1;
        println!("{}", message);
        return;
    }

    /// This function checks the balance of a given account and prints it to the console.
    /// 
    /// Arguments:
    /// 
    /// * `account_id`: The `account_id` of the `Account` who's balanced is being checked
    pub fn check_balance(&self, account_id: i32) {
        let account = &self.accounts[account_id as usize];
        let account_lock = account.lock.read().unwrap();
        let message = format!(
            "ID# {:2} | {:9}", account.account_id, account.balance
        );
        drop(account_lock);
        println!("{}", message);
    }
}
