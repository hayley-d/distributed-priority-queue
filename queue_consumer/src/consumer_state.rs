use std::sync::Arc;

use tokio::sync::Mutex;

use crate::job_management::Job;
use crate::min_heap::MinHeap;

pub struct ConsumerState {
    nodes: Vec<String>,
    heap: MinHeap,
    lamport_timestamp: i64,
}

impl ConsumerState {
    pub fn new(nodes: Vec<String>) -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(ConsumerState {
            nodes,
            heap: MinHeap::new(0.5),
            lamport_timestamp: 0,
        }));
    }

    pub fn insert_job(job: Job) {
        todo!()
    }
}
