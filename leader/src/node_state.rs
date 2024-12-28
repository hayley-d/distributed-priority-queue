use std::sync::Arc;

use log::error;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::db::connect_to_db;

pub struct NodeState {
    lamport_timestamp: Arc<Mutex<u64>>,
    node_id: Arc<u64>,
    db: Arc<Mutex<Client>>,
}

impl NodeState {
    pub async fn new() -> Self {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        let node_id: u64 = match std::env::args().collect::<Vec<String>>().get(1) {
            Some(id) => match id.parse::<u64>() {
                Ok(i) => i,
                Err(_) => std::process::exit(1),
            },
            None => std::process::exit(1),
        };

        let db = match connect_to_db().await {
            Ok(d) => d,
            Err(_) => {
                error!("Failed to connect to database");
                std::process::exit(1);
            }
        };

        return NodeState {
            lamport_timestamp: Arc::new(Mutex::new(0)),
            node_id: Arc::new(node_id),
            db: Arc::new(Mutex::new(db)),
        };
    }
}
