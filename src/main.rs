use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::env;
use std::io;
pub mod model;
use model::*;

fn read_and_process_transactions(path: &str) -> Result<HashMap<u16, AccountData>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut accounts: HashMap<u16, AccountData> = HashMap::new();

    // tx_client_container is: HashMap<TransactionId, (ClientId, TransactionState, Option<Amount>)>
    let mut tx_client_container: HashMap<u32, (u16, TransactionState, Option<Decimal>)> =
        HashMap::new();

    for result in reader.deserialize::<Transaction>() {
        let transaction = result?;
        let mut account = accounts.entry(transaction.client).or_insert({
            AccountData {
                available: Decimal::new(0, 4),
                held: Decimal::new(0, 4),
                total: Decimal::new(0, 4),
                locked: false,
            }
        });
        // Check wheter it is 'standard' operation and process it for client in actual transaction.
        if transaction.transaction_type.standard() {
            tx_client_container.insert(
                transaction.tx,
                (
                    transaction.client,
                    transaction.transaction_type.clone(),
                    transaction.amount,
                ),
            );
            transaction.process(&mut account, transaction.amount);
        } else {
            // Get already saved client based on Transaction ID and process transactions for him.
            if let Some(client) = tx_client_container.get_mut(&transaction.tx) {
                // Get already saved clients account based on client u16 identifier.
                if let Some(account) = accounts.get_mut(&client.0) {
                    if transaction.transaction_type.fulfill_dispute()
                        && client.1 == TransactionState::Dispute
                    {
                        transaction.process(account, client.2);
                    } else {
                        client.1 = TransactionState::Dispute;
                        transaction.process(account, client.2);
                    }
                }
            }
        }
    }
    Ok(accounts)
}

fn write_output_to_std(accounts: HashMap<u16, AccountData>) -> Result<(), csv::Error> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["client", "available", "held", "total", ""])?;
    for account in accounts {
        writer.write_record(&[
            account.0.to_string(),
            account.1.available.to_string(),
            account.1.held.to_string(),
            account.1.total.to_string(),
            account.1.locked.to_string(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn usage() {
    println!("Program has to be executed as expected:");
    println!("\'cargo run -- transactions.csv > accounts.csv\'")
}

fn main() -> Result<(), csv::Error> {
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1) {
        let transactions = read_and_process_transactions(filename)?;
        write_output_to_std(transactions)?;
    } else {
        usage()
    }
    Ok(())
}
