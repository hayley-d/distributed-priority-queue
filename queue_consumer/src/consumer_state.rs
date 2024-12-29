use std::sync::Arc;

use tokio::sync::Mutex;

use crate::job_management::long_polling_service_client::LongPollingServiceClient;
use crate::job_management::{Job, PollJobRequest};
use crate::min_heap::MinHeap;

pub struct ConsumerState {
    consumer_id: i32,
    nodes: Vec<String>,
    heap: MinHeap,
    lamport_timestamp: i64,
    timeout: i32
}

impl ConsumerState {
    pub fn new(nodes: Vec<String>) -> Arc<Mutex<Self>> {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        let consumer_id: i32 = match std::env::args().collect::<Vec<String>>().get(1) {
            Some(id) => match id.parse::<i32>() {
                Ok(i) => i,
                Err(_) => std::process::exit(1),
            },
            None => std::process::exit(1),
        };

        let timeout: i32 = match std::env::args().collect::<Vec<String>>().get(1) {
            Some(id) => match id.parse::<i32>() {
                Ok(i) => i,
                Err(_) => 5,
            },
            None => 5,
        };


        return Arc::new(Mutex::new(ConsumerState {
            consumer_id,
            nodes,
            heap: MinHeap::new(0.5),
            lamport_timestamp: 0,
            timeout
        }));
    }

    pub fn insert_job(&mut self, job: Job) {
        todo!()
    }
}

pub struct LocalLongPollService {
    consumer_state: Arc<Mutex<ConsumerState>>,
}

impl LocalLongPollService {
    pub async fn get_job(&mut self) {
        let gaurd = &self.consumer_state.lock().await;

        let request : PollJobRequest {
            consumer_id: gaurd.consumer_id,
            timeout_seconds: gaurd.timeout
        }
        let mut responses = Vec::new();

        for node in &gaurd.nodes {
            let mut client = LongPollingServiceClient::connect(node.to_string())
                .await
                .unwrap();
            let response = client.poll(paxos_prepare.clone()).await;
            responses.push(response);
        }

        // Check the first successful PaxosPromise response
        let mut paxos_promise = None;
        for response in responses {
            if let Ok(promise) = response {
                if promise.get_ref().accepted_value == 1 {
                    paxos_promise = Some(promise);
                    break;
                }
            }
        }

        if paxos_promise.is_none() {
            error!(
                "Paxos prepared failed with proposal number = {}",
                proposal_number
            );
            return Err(Status::internal("Paxos prepared failed"));
        }
    }
}
