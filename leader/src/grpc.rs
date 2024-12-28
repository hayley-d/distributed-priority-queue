use crate::job_management::job_service_server::JobService;
use crate::job_management::{EnqueueRequest, Job, JobRequest, JobResponse, PaxosPrepare};
use crate::node_state::NodeState;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use tonic::{transport::Server, Request, Response, Status};
pub mod proto {
    tonic::include_proto!("job_management");
}

pub struct LocalJobService {
    node_state: Arc<Mutex<NodeState>>,
}

impl LocalJobService {
    pub async fn new(node_state: Arc<Mutex<NodeState>>) -> Self {
        LocalJobService { node_state }
    }
}

#[tonic::async_trait]
impl JobService for LocalJobService {
    // EnqueueJob RPC method
    async fn enqueue_job(
        &self,
        request: Request<EnqueueRequest>,
    ) -> Result<Response<JobResponse>, Status> {
        let enqueue_request = request.into_inner();
        let priority = enqueue_request.priority;
        let payload = enqueue_request.payload;

        let paxos_prepare = PaxosPrepare {
            proposal_number: self.node_state.lock().await.increment_timestamp(),
        };

        let mut responses = Vec::new();

        for follower in self.node_state.lock().await.followers {
            let mut client = PaxosClient::connect(follower).await.unwrap();
        }

        Ok(Response::new(JobResponse {
            job: Some(Job {
                job_id,
                priority,
                payload,
            }),
        }))
    }

    async fn get_task(
        &self,
        request: Request<JobRequest>,
    ) -> Result<Response<JobResponse>, Status> {
        todo!()
    }
}
