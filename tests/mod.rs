#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use banking_example::model::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_deposit() {
        let transaction = Transaction {
            transaction_type: TransactionState::Deposit,
            client: 1,
            tx: 1,
            amount: Some(Decimal::new(100, 0)),
        };

        let mut account_data = AccountData {
            available: Decimal::new(100, 0),
            held: Decimal::new(0, 0),
            total: Decimal::new(100, 0),
            locked: false,
        };
        transaction.process(&mut account_data, Some(Decimal::new(100, 0)));
        assert_eq!(account_data.available, Decimal::new(200, 0));
        assert_eq!(account_data.held, Decimal::new(0, 0));
    }

    #[test]
    fn test_chargeback() {
        let transaction = Transaction {
            transaction_type: TransactionState::Chargeback,
            client: 1,
            tx: 1,
            amount: Some(Decimal::new(100, 0)),
        };

        let mut account_data = AccountData {
            available: Decimal::new(100, 0),
            held: Decimal::new(0, 0),
            total: Decimal::new(100, 0),
            locked: false,
        };
        transaction.process(&mut account_data, Some(Decimal::new(100, 0)));
        assert_eq!(account_data.locked, true);
    }
}
