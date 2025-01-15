use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct ManagerState {
    pub lamport_timestamp: i64,
    pub manager_id: i32,
    pub nodes: Vec<String>,
}

impl ManagerState {
    pub fn new(nodes: Vec<String>) -> Arc<Mutex<Self>> {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        let manager_id: i32 = match std::env::args().collect::<Vec<String>>().get(1) {
            Some(id) => match id.parse::<i32>() {
                Ok(i) => i,
                Err(_) => {
                    error!(target:"error_logger","Failed to parse manager id: Manager id must be of type u64");
                    std::process::exit(1);
                }
            },
            None => {
                error!(target:"error_logger","No manager id provided in command line arguments: could not start server");
                std::process::exit(1);
            }
        };

        return Arc::new(Mutex::new(ManagerState {
            lamport_timestamp: 0,
            manager_id,
            nodes,
        }));
    }

    pub fn increment_time(&mut self) -> i64 {
        let temp: i64 = self.lamport_timestamp;
        self.lamport_timestamp += 1;
        return temp;
    }
}
