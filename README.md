# 377_ledger_in_rust

rust unfortunately ties ownership of all elements within a vector to the vector itself
this means that one thread cannot own accounts[0] while another thread concurrenty owns accounts[1]

therefore, this producer/consumer project is less concurrent than the original project in C++
due to rust ownership rules, the whole bank has to be locked rather than each individual account
