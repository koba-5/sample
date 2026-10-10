use crate::framework::transaction_manager::executor::Executor;
use crate::framework::transaction_manager::transaction::Transaction;

pub mod executor;
pub mod transaction;
mod status;

pub fn transaction_manager<F>(f: F)
where
    F: FnOnce(&Executor) -> Result<(), ()>,
{
    let transaction = &mut Transaction::new();
    let executor = &Executor::new(transaction);
    match f(executor) {
        Ok(()) => {transaction.commit()}
        Err(()) => {transaction.rollback()}
    }

}
