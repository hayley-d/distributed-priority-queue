use crate::db::connect_to_db;
use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use tonic::{Code, Status};

pub struct NodeState {
    pub lamport_timestamp: i32,
    pub node_id: u64,
    pub db: Client,
    pub followers: Vec<String>,
}

impl NodeState {
    pub async fn new(followers: Vec<String>) -> Arc<Mutex<Self>> {
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
                eprintln!("Failed to connect to the database, could not start server");
                std::process::exit(1);
            }
        };

        return Arc::new(Mutex::new(NodeState {
            lamport_timestamp: 0,
            node_id,
            db,
            followers,
        }));
    }

    // Returns the current time and increments it
    pub fn increment_timestamp(&mut self) -> i32 {
        let temp: i32 = self.lamport_timestamp;
        self.lamport_timestamp += 1;
        return temp as i32;
    }

    pub async fn insert_job(&mut self, priority: u32, payload: Vec<u8>) -> Result<u64, Status> {
        let query = &self
            .db
            .prepare("INSERT INTO jobs (priority, payload) VALUES ($1,$2) RETURNING job_id")
            .await
            .map_err(|_| {
                error!("Failed to create INSERT query");
                return Status::new(Code::Internal, format!("Failed to create INSERT query"));
            });

        let row = &self
            .db
            .query_one(&query, &[&priority, &payload])
            .await
            .map_err(|_| {
                error!("Failed to run INSERT query");
                return Status::new(Code::Internal, format!("Failed to run INSERT query"));
            })?;

        let job_id: i64 = row.get(0).map_err(|_| {
            error!("Failed to get job_id from newly created job");
            return Status::new(
                Code::Internal,
                format!("Failed to get job_id from newly created job"),
            );
        })?;

        // Increment logical time
        let time = self.increment_timestamp();

        println!("Inserted job with job_id {} into jobs table", job_id);

        return Ok(job_id);
    }
}
