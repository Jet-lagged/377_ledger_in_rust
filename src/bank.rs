use std::sync::{Mutex, Arc, RwLock};
#[derive(Debug)]
pub(crate) struct Bank {
    _num: i32,
    accounts: Vec<Account>,
    num_succ: Arc<Mutex<i32>>,
    num_fail: Arc<Mutex<i32>>,
}

#[derive(Debug)]
struct Account {
    account_id: i32,
    balance: i32,
    lock: RwLock<()>,
}

// TODO: MAKE A NOTE ABOUT THIS SOMEHOW
// say that it is currently hard coded that the code formats account 
// and ledger ids with min 2 char and money stuff with min 8 char
impl Bank {
    /// Constructs a new Bank:: Bank object.
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

    /// Prints account information for all accounts in the bank
    pub fn print_account(&self) {
        for account in &self.accounts {
            let _lock = account.lock.read().unwrap();
            println!("ID# {:2} | {:9}", account.account_id, account.balance);
        } // auto drops lock

        let num_succ = self.num_succ.lock().unwrap();
        let num_fail = self.num_fail.lock().unwrap();
        println!("Success: {:2} Fails: {:2}", *num_succ, *num_fail);
    }

    /// deposits amount into account with id, does not fail
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

    /// withdraws amount from account with id, fails if balance < amount
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

    /// transfers amount from account with src_id to dest_id
    /// locks account with lower id first to prevent deadlock
    /// fails if src_acnt.balance < amount
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

    /// prints the id and balance of an account with read-only access
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
