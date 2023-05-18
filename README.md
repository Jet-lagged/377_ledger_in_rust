# 377 Final Project: Ledger in Rust
[Presentation](https://www.youtube.com/watch?v=VIDEO_ID)

## About
Rust implementation of a multi-threaded Producer/Consumer model of a bank based on [Pr4: Producer/Consumer](https://umass-cs-377.github.io/docs/projects/prodcon/) For a final project in an OS class with Timothy Richards. The gist of the goal is to take a ledger list, parse it, process the transactions within a bank with bank accounts, and to do so with multiple threads using shared-state concurrency. Each line in the ldeger list is made into a `Ledger` struct and put into a vector, eventually being processed by a number of worker threads who do the depositing or withdrawl of money from a bank account in the bank. The program only processes 1 ledger list before terminating.

## Getting Started
Make sure you have [Rust](https://www.rust-lang.org/tools/install) Installed.
Navigate to the directory you want to put this.
```
$ git clone https://github.com/Jet-lagged/377_ledger_in_rust
$ cd 377_ledger_in_rust
$ cargo build
```
Thats it!

## Functionality
### How to run the code
Navigate to the root directory that contains the `src` and `ledgers` directories and input `cargo run <num_of_threads> <ledger_file> <sleep: bool>` into the command line.
- `<num_of_threads>` should be an integer value and is used to determine how many worker threads will process the ledgers.
- `<ledger_file>` should be the name of a file in the `ledgers` directory that is to be processed.
- `<sleep>` should be a boolean and is used more for demonstration purposes than anything. It will cause all threads to sleep for a short time after obtaining a ledger, but before it actually processes it. This delay is mean to showcase the multi-threaded nature of this program. 

### Ledger lists
The 'ledgers' directory hosts the possible inputs for the ledger_file and each line for every file should be formatted as such:
```
FROM_ID TO_ID AMOUNT MODE
```
- `FROM_ID` is an integer that represents the ID of the account sending this transation
- `TO_ID` is an integer that represents the ID of the account recieving this transation (only used in transfer)
- `AMOUNT` is an integer that represents the amount of money in the transation
- `MODE` is a single character: either `'D'`, `'W'`, `'T'`, or `'C'`, representing what kind of transation the line is

For instance, consider the following line in ledger.txt:
```
...
4 3 400 T
...
```
This line says to take $400 out of Account[4] and add it to Account[3].
No lines in the file should be empty or formatted incorrectly.
Here is an example output from one run of a ledger file
```
ID#  0 |         0
ID#  1 |         0
ID#  2 |         0
ID#  3 |         0
ID#  4 |         0
ID#  5 |         0
ID#  6 |         0
ID#  7 |         0
ID#  8 |         0
ID#  9 |         0
Success:  0 Fails:  0
Worker  0 completed ledger  0:     deposit       200 into account  0
Worker  0 completed ledger  1:    balance=       200  for account  0
Worker  0 completed ledger  2:    withdraw       100 from account  0
Worker  0 completed ledger  3:    balance=       100  for account  0
Worker  0 -FAILED-  ledger  4:    withdraw       200 from account  0
Worker  0 completed ledger  5:    balance=       100  for account  0
Worker  0 completed ledger  6:     deposit       500 into account  1
Worker  0 completed ledger  7:    balance=       500  for account  1
Worker  0 completed ledger  8:    transfer       200 from account  1 to account  0
Worker  0 -FAILED-  ledger  9:   Ledger Error: no account with inputted ID
ID#  0 |       300
ID#  1 |       300
ID#  2 |         0
ID#  3 |         0
ID#  4 |         0
ID#  5 |         0
ID#  6 |         0
ID#  7 |         0
ID#  8 |         0
ID#  9 |         0
Success:  8 Fails:  2
```

### Bank
- `struct Bank`: the bank object that holds all the bank `accounts`
- `new`: the constructor for a bank object. Takes parameter  `num: i32` as creates `num` accounts with incrementing IDs and balances of 0. 
- `print_account`: prints all accounts' IDs and their associated balance as well as how many successful and failed ledgers have happened
- `deposit`, `withdraw`, `transfer`, and `check_balance` documentation can be found within the code as their functions are self explanatory

### Ledger
- Documentation on the `Ledger` struct and `Mode` can be found in the code
- `read_ledger_file` takes in a filename of a file from the `ledgers` directory and parses it into a Vec<Ledger> to be returned
- `init_bank` is the function that puts everything else together. A `Bank` struct is created and a `ledger_list` read, both with Arc<Mutex<>> wrappers, which are the locks that allow this system to be multithreaded. `num_workers` amount of worker threads are created to process the ledgers, each first locking the ledger list and removing the first `Ledger before dropping the lock. Then the worker locks the bank (and therefore the bank accounts) and then performs the transaction, dropping the bank lock once the ledger is processed. 

## Design Considerations
Rust unfortunately ties ownership of all elements within a vector to the vector itself.
This means that one thread cannot own accounts[0] while another thread concurrenty owns accounts[1] as is done in the [original project](https://umass-cs-377.github.io/docs/projects/prodcon/) in C++.

Therefore, this producer/consumer project is less concurrent than the original project.
Due to rust ownership rules, the whole bank has to be locked rather than each individual account.
If you view my commit history, you might see that early on, I tried to create a Bounded Buffer and multiple reader threads before eventually realizing that structs aren't borrowed and shared so easily in Rust. Rust puts a bigger emphasis on channels when it comes to multi-threading, a system where threads `send` data between each other rather than `sharing` the data in a centralized location. (See Chapter 16 of The Rust Programming Language on [Message Passing](https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html) and [Shared-State Concurrency](https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html)
  
In general, locks have been moved from the global scope in the original project, as well as from within the `Account` struct to being defined within the `init_bank` function itself, where Arc<Mutex<>> wrappers are placed around the `Bank` and `ledger_list`.

A check_balance function has been added, which prints out the balance of an account. It takes `worker_id` `ledger_id` and `account_id` as parameters and prints the balance of the account with `account_id`
  

