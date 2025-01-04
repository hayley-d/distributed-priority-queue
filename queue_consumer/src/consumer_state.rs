use std::sync::Arc;

use tokio::sync::Mutex;

use crate::job_management::long_polling_service_client::LongPollingServiceClient;
use crate::job_management::{Job, PollJobRequest, PollJobResponse};
use crate::min_heap::MinHeap;

/// Consumer state represented with:
/// consumer_id: The id of the consumer provided in command line arguments at startup.
/// nodes: A list of nodes that the consumer pulls from to get jobs.
/// heap: A loacal min heap implementation for fetched jobs.
/// lamport_timestamp: A logical clock
/// timeout: Specified timeout for long polling
///
/// # Example
/// ```
/// use queue_consumer::consumer_state;
/// let aging_factor: f32 = 0.5;
///
/// let state: ConsumerState = {
///     consumer_id: 1,
///     nodes : vec!["http://node1", "http://node2"],
///     heap: MinHeap::new(aging_factor),
///     lamport_timestamp: 0,
///     timeout: 30
/// }
/// ```
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

        // create a request to get jobs from the nodes
        let request: PollJobRequest = PollJobRequest {
            consumer_id: gaurd.consumer_id,
            timeout_seconds: gaurd.timeout,
        };

        let mut responses: Vec<Result<tonic::Response<PollJobResponse>, tonic::Status>> = vec![];

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
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
