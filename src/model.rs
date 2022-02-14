use strum_macros::Display;
use serde::{Deserialize, Serialize};
use rust_decimal::prelude::*;

#[derive(Display, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TransactionState {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl TransactionState {
    pub fn standard(&self) -> bool {
        match *self {
            TransactionState::Deposit | TransactionState::Withdrawal => true,
            TransactionState::Dispute
            | TransactionState::Resolve
            | TransactionState::Chargeback => false,
        }
    }

    pub fn fulfill_dispute(&self) -> bool {
        match *self {
            TransactionState::Deposit
            | TransactionState::Withdrawal
            | TransactionState::Dispute => false,
            TransactionState::Resolve | TransactionState::Chargeback => true,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub transaction_type: TransactionState,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<Decimal>,
}

impl Transaction {
    // 'amount' parameter could be taken from struct itself (self.amount)
    // but in some cases we want to increase/decrease amount of money
    // from already processed transaction, that's why it has to be given as an argument.
    pub fn process(&self, account: &mut AccountData, amount: Option<Decimal>) {
        if let Some(amount) = amount {
            match self.transaction_type {
                TransactionState::Deposit => {
                    account.available += amount;
                    account.total += amount;
                }
                TransactionState::Withdrawal => {
                    if amount <= account.available {
                        account.available -= amount;
                        account.total -= amount;
                    }
                }
                TransactionState::Dispute => {
                    account.available -= amount;
                    account.held += amount;
                }
                TransactionState::Resolve => {
                    account.held -= amount;
                    account.available += amount;
                }
                TransactionState::Chargeback => {
                    account.held -= amount;
                    account.total -= amount;
                    account.locked = true;
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountData {
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

// I'm not sure if there are any pros of turning HashMap<client, AccountData>
// into Account struct except for "easier" serialization to csv?? (this could cost - re-writing data).
// Left here to show that it's possible, but not reccomended for this task's purpose I assume...
#[allow(dead_code)]
#[derive(Serialize)]
pub struct Account {
    pub client: u16,
    #[serde(flatten)]
    pub account: AccountData,
}
