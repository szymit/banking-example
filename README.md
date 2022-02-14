# Usage
```
cargo run -- transactions.csv > accounts.csv
```

# Test 
Program has been tested agains included data and also with included (by myself) 'dispute' and 'chargeback' operations. <br />
It can be tested using command from "Usage" part of Readme.

It also has two basic unit tests mainly for processing operations. <br />
They can be executed using ```cargo test```. <br />

CSVs output is the thing that could be checked here based on known CSV provided. It would need a few changes in code to provide function for single transaction processing. <br />

# Safety - error handling
As long as I understood some instructions about erorrs. <br />

I assumed that some cases based on wrong values on input shouldn't be handled as a Rust Error and it should be just ommited. <br />
If not, then I would implement my Error Trait that could also accept csv::Error and my own kind of errors. <br />
I haven't found to throw error for any kind of transactions because at the end I wanted to throw all the proper ones to the std output. <br />

To sum things up. The only error that is handled is from csv::Error. <br />

# Efficiency
At first I wanted to put all transactions into a ```Vec<Transaction>``` and then use it as an argument ```&[Transaction]``` in helper function to mainly process them. <br />
But for large files it would take a lot of resources to keep it in memory, so I focused on reading a transaction.csv line by line and processing it at once. <br />

If those CSVs would come from thousands of concurrent TCP strams, I would need to manage them with a help of ```Mutex``` struct from std or tokio - based if there would be sync or async calls. <br />
```Mutex``` would be used for writing session. Reading and processing data could be handled by a separate spawned processes using ```Tokio``` crate. <br />

# Maintainability 
Using some helper struct for ```let mut tx_client_container: HashMap<u32, (u16, TransactionState, Option<Decimal>)>``` I think might be a better option for some people. <br />
