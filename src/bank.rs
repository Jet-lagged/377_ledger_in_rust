use std::sync::{Mutex, Arc};

#[derive(Debug)]
pub(crate) struct Bank {
    _num: u32,
    accounts: Vec<Account>,
    num_succ: Arc<Mutex<u32>>,
    num_fail: Arc<Mutex<u32>>,
}

#[derive(Debug)]
struct Account {
    account_id: u32,
    balance: i32,
    lock: Mutex<()>,
}

impl Bank {
    /// Constructs a new Bank:: Bank object.
    pub fn new(_num: u32) -> Self {
        let mut accounts = Vec::with_capacity(_num as usize);
        for i in 0.._num {
            accounts.push(Account {
                account_id: i,
                balance: 0,
                lock: Mutex::new(()),
            });
        }
        let num_succ = Arc::new(Mutex::new(0 as u32));
        let num_fail = Arc::new(Mutex::new(0 as u32));
        Bank {
            _num,
            num_succ,
            num_fail,
            accounts,
        }
    }

    /// Prints account information for all accounts in the bank
    pub fn print_account(&self) {
        for account in &self.accounts {
            let lock = account.lock.lock().unwrap();
            println!("ID# {} | {}", account.account_id, account.balance);
            drop(lock);
        }

        let num_succ = self.num_succ.lock().unwrap();
        let num_fail = self.num_fail.lock().unwrap();
        println!("Success: {} Fails: {}", *num_succ, *num_fail);
    }

    pub fn deposit(&mut self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32){
        // Success (always)
        let mut account = &mut self.accounts[account_id as usize];
        let account_lock = account.lock.lock().unwrap();
        account.balance += amount;
        let message = format!(
            "Worker {} completed ledger {}: deposit {} into account {}",
            worker_id, ledger_id, amount, account_id
        );
        drop(account_lock);
        let mut num_succ = self.num_succ.lock().unwrap();
        *num_succ += 1;
        println!("{}", message);
        return;
    }

    pub fn withdraw(&mut self, worker_id: i32, ledger_id: i32, account_id: i32, amount: i32) {
        let mut account = &mut self.accounts[account_id as usize];
        let account_lock = account.lock.lock().unwrap();

        // Fail 
        if account.balance < amount {
            let message = format!(
                "Worker {} failed to complete ledger {}: withdraw {} from account {}",
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
            "Worker {} completed ledger {}: withdraw {} from account {}",
            worker_id, ledger_id, amount, account_id
        );
        drop(account_lock);
        let mut num_succ = self.num_succ.lock().unwrap();
        *num_succ += 1;
        println!("{}", message);
        return;
    }
}
