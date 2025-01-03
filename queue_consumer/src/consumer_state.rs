use std::sync::Arc;

use tokio::sync::Mutex;

use crate::job_management::long_polling_service_client::LongPollingServiceClient;
use crate::job_management::{Job, PollJobRequest, PollJobResponse};
use crate::min_heap::MinHeap;

pub struct ConsumerState {
    consumer_id: i32,
    nodes: Vec<String>,
    heap: MinHeap,
    lamport_timestamp: i64,
    timeout: i32,
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
            timeout,
        }));
    }

    pub fn insert_job(&mut self, job: Job) {
        let time: i64 = self.increment_time();
        self.heap
            .insert(job.priority as u32, job.job_id as u64, time as u64);
    }

    fn increment_time(&mut self) -> i64 {
        let temp = self.lamport_timestamp;
        self.lamport_timestamp += 1;
        return temp;
    }
}

pub struct LocalLongPollService {
    consumer_state: Arc<Mutex<ConsumerState>>,
}

impl LocalLongPollService {
    pub async fn get_jobs(&mut self) {
        let gaurd = &mut self.consumer_state.lock().await;

        let request: PollJobRequest = PollJobRequest {
            consumer_id: gaurd.consumer_id,
            timeout_seconds: gaurd.timeout,
        };

        let mut responses = Vec::new();

        for node in &gaurd.nodes {
            let mut client = LongPollingServiceClient::connect(node.to_string())
                .await
                .unwrap();
            let response = client.poll(request.clone()).await;
            responses.push(response);
        }

        for response in responses {
            match response {
                Ok(res) => {
                    let res = res.into_inner().clone();
                    let (success, option_job) = (&res.success.clone(), &res.job.clone());

                    if *success {
                        let job = match option_job {
                            Some(job) => job,
                            None => continue,
                        };

                        gaurd.insert_job(job.clone());
                    } else {
                        continue;
                    }
                }
                Err(_) => continue,
            }
        }
    }
}
