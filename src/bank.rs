
/// The `Bank` struct contains a number, a vector of `Account` structs, and two `Arc<Mutex<i32>>`
/// variables for tracking successful and failed operations.
/// 
/// Properties:
/// 
/// * `num`: Used to indicate how many accounts are created.
/// * `accounts`: a vector of `Account` structs that belong to the `Bank`. It stores all
/// the accounts that the bank manages.
/// * `num_succ`: a shared counter that keeps track of the number of successful operations
/// performed by the bank. It is an `Arc` (atomic reference counting) wrapped `Mutex` that allows
/// multiple threads to safely access and modify the counter.
/// * `num_fail`: a shared counter that keeps track of the number of failed operations
/// performed by the bank. It is an `Arc` (atomic reference counting) wrapped `Mutex` that allows
/// multiple threads to safely access and modify the counter.
pub struct Bank {
    pub num: i32,
    accounts: Vec<Account>,
    num_succ: i32,
    num_fail: i32,
}

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
}


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
    pub fn new(num: i32) -> Self {
        let mut accounts = Vec::with_capacity(num as usize);
        for i in 0..num {
            accounts.push(Account {
                account_id: i,
                balance: 0,
            });
        }
        Bank {
            num,
            num_succ: 0,
            num_fail: 0,
            accounts,
        }
    }

    /// Prints the account IDs and balances of all accounts in a bank, as well as
    /// the number of successful and failed transactions.
    pub fn print_account(&self) {
        for account in &self.accounts {
            println!("ID# {:2} | {:9}", account.account_id, account.balance);
        }
        println!("Success: {:2} Fails: {:2}", self.num_succ, self.num_fail);
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
        if account_id < 0 || account_id >= self.num {
            self.num_fail += 1;
            println!("Worker {:2} -FAILED-  ledger {:2}:   Ledger Error: no account with inputted ID", worker_id, ledger_id);
            return;
        }
        // Success
        let mut account = &mut self.accounts[account_id as usize];
        account.balance += amount;
        let message = format!(
            "Worker {:2} completed ledger {:2}:     deposit {:9} into account {:2}",
            worker_id, ledger_id, amount, account_id
        );
        self.num_succ += 1;
        println!("{}", message);
        return;
    }

    /// Withdraws a specified amount from a specified account, updating
    /// the account balance and logging the transaction.
    /// 
    /// Arguments:
    /// 
    /// * `worker_id`: An integer representing the ID of the worker performing the withdrawal operation.
    /// * `ledger_id`: The ID of the ledger from which the withdrawal is being made.
    /// * `account_id`: The `account_id` of the `Account` from which the withdrawal is being made.
    /// * `amount`: The amount of money to withdraw from the `Account`.
    pub fn withdraw(&mut self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) {
        if account_id < 0 || account_id >= self.num {
            self.num_fail += 1;
            println!("Worker {:2} -FAILED-  ledger {:2}:   Ledger Error: no account with inputted ID", worker_id, ledger_id);
            return;
        }

        let mut account = &mut self.accounts[account_id as usize];
        // Fail 
        if account.balance < amount {
            let message = format!(
                "Worker {:2} -FAILED-  ledger {:2}:    withdraw {:9} from account {:2}",
                worker_id, ledger_id, amount, account_id
            );
            self.num_fail += 1;
            println!("{}", message);
            return;
        }

        // Success
        account.balance -= amount;
        let message = format!(
            "Worker {:2} completed ledger {:2}:    withdraw {:9} from account {:2}",
            worker_id, ledger_id, amount, account_id
        );
        self.num_succ += 1;
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
        let comp = src_id as i32;
        let comp2 = dest_id as i32;
        if comp < 0 || comp >= self.num  || comp2 < 0 || comp2 >= self.num{
            self.num_fail += 1;
            println!("Worker {:2} -FAILED-  ledger {:2}:   Ledger Error: no account with inputted ID", worker_id, ledger_id);
            return;
        }

        // Handle tranfering money to oneself
        if src_id == dest_id {
            let message = format!(
                "Worker {:2} -FAILED-  ledger {:2}:    transfer {:9} from account {:2} to account {:2}",
                worker_id, ledger_id, amount, src_id, dest_id
            );
            self.num_fail += 1;
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
        
        // Fail
        if src_acnt.balance < amount {
            let message = format!(
                "Worker {:2} -FAILED-  ledger {:2}:    transfer {:9} from account {:2} to account {:2}",
                worker_id, ledger_id, amount, src_id, dest_id
            );
            self.num_fail += 1;
            println!("{}", message);
            return;
        }

        // Success
        src_acnt.balance -= amount;
        dest_acnt.balance += amount;
        let message = format!(
            "Worker {:2} completed ledger {:2}:    transfer {:9} from account {:2} to account {:2}",
            worker_id, ledger_id, amount, src_id, dest_id
        );
        self.num_succ += 1;
        println!("{}", message);
        return;
    }

    /// Checks the balance of a specific account and prints a message with the worker ID,
    /// ledger ID, account balance, and account ID.
    /// 
    /// Arguments:
    /// 
    /// * `worker_id`: An integer representing the ID of the worker who completed the ledger.
    /// * `ledger_id`: The ID of the ledger for which the balance is being checked.
    /// * `account_id`: The ID of the account for which the balance needs to be checked.
    pub fn check_balance(&mut self, worker_id: i32, ledger_id: i32, account_id: i32) {
        if account_id < 0 || account_id >= self.num {
            self.num_fail += 1;
            println!("Worker {:2} -FAILED-  ledger {:2}:   Ledger Error: no account with inputted ID", worker_id, ledger_id);
            return;
        }

        let account = &self.accounts[account_id as usize];
        let message = format!(
            "Worker {:2} completed ledger {:2}:    balance= {:9}  for account {:2}", worker_id, ledger_id, account.balance, account.account_id
        );
        self.num_succ += 1;
        println!("{}", message);
    }
}
