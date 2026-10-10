use crate::framework::transaction_manager::executor::Executor;

pub struct UserDBRepository<'a> {
    executor: &'a Executor<'a>
}

impl <'a> UserDBRepository<'a> {
    pub fn new(executor: &'a Executor) -> Self{
        Self {
            executor
        }
    }

    pub fn find(&self, id: i64) -> Result<(),()>{
        self.executor.query(format!("SELECT * FROM USER WHERE id = '{}';", id))
    }

    pub fn create(&self) -> Result<(),()>{
        self.executor.execute("SELECT 1 FROM USER;".to_string())
    }

    pub fn update(&self, id: i64) -> Result<(),()>{
        self.executor.execute(format!("UPDATE USER SET DUMMY = '1' WHERE ID = '{}';",id))
    }

    pub fn delete(&self, id: i64) -> Result<(),()>{
        self.executor.execute(format!("DELETE FROM USER WHERE ID = '{}';",id))
    }
}

