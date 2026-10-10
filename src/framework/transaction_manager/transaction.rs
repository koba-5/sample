use rand::random;

pub struct Transaction {
    status: super::status::Status,
}

impl Transaction {
    pub fn new() -> Self {
        println!("[SQL][BEGIN]");
        Self {
            status: super::status::Status::Connected,
        }
    }

    pub fn query(&self, sql: String) -> Result<(), ()> {
        if (random::<f64>() > 0.5) {
            Ok(println!("[SQL][QUERY] {}", sql))
        } else {
            Err(println!("[SQL][QUERY][ERROR] {}", sql))
        }
    }

    pub fn execute(&self, sql: String) -> Result<(), ()> {
        if (random::<f64>() > 0.5) {
            Ok(println!("[SQL][EXECUTE] {}", sql))
        } else {
            Err(println!("[SQL][EXECUTE][ERROR] {}", sql))
        }
    }

    pub fn commit(&mut self) {
        self.status = super::status::Status::Disconnected;
        println!("[SQL][COMMIT]");
    }

    pub fn rollback(&mut self) {
        self.status = super::status::Status::Disconnected;
        println!("[SQL][ROLLBACK]");
    }
}
