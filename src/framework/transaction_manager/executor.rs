use crate::framework::transaction_manager::transaction::Transaction;

pub struct Executor<'a> {
    transaction: &'a Transaction,
}

impl<'a> Executor<'a> {
    pub fn new(transaction: &'a Transaction) -> Self {
        Self { transaction }
    }

    pub fn query(&self, sql: String) -> Result<(), ()> {
        self.transaction.query(sql)
    }

    pub fn execute(&self, sql: String) -> Result<(), ()> {
        self.transaction.execute(sql)
    }
}
