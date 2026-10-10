use crate::framework::config::Config;
use crate::framework::transaction_manager::transaction_manager;
use crate::repository::db::user::UserDBRepository;

mod framework;
mod repository;

fn main() {
    let config = Config::new();
    transaction_manager(|executor| {
        let db_repository = UserDBRepository::new(executor);
        db_repository.find(1)?;
        db_repository.find(1)?;
        db_repository.find(1)?;
        db_repository.find(1)?;
        db_repository.find(1)?;
        Ok(())
    });
    println!("The server port is {}.", config.server.port);
}
