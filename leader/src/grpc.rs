use crate::job_management::job_service_server::JobService;
use crate::job_management::paxos_service_client::PaxosServiceClient;
use crate::job_management::{
    EnqueueRequest, Job, JobRequest, JobResponse, PaxosCommit, PaxosPrepare, PaxosPropose,
};
use crate::node_state::NodeState;
use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

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
        let proposal_number: i32 = self.node_state.lock().await.increment_timestamp();
        let mut job_id: i64 = -1;
        let paxos_prepare = PaxosPrepare { proposal_number };

        let mut responses = Vec::new();

        for follower in &self.node_state.lock().await.followers {
            let mut client = PaxosServiceClient::connect(follower).await.unwrap();
            let response = client.prepare(paxos_prepare.clone()).await;
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

        job_id = self
            .node_state
            .lock()
            .await
            .insert_job(priority as u32, payload)
            .await? as i64;

        let paxos_propose = PaxosPropose {
            proposal_number,
            proposed_job: Some(Job {
                job_id,
                priority,
                payload: payload.clone(),
            }),
        };

        let mut responses = Vec::new();
        for follower in &self.node_state.lock().await.followers {
            let mut client = PaxosServiceClient::connect(follower).await.unwrap();
            let response = client.propose(paxos_propose.clone()).await;
            responses.push(response);
        }

        let mut paxos_propose = None;
        for response in responses {
            if let Ok(propose) = response {
                if propose.get_ref().proposal_number == proposal_number {
                    paxos_propose = Some(propose);
                    break;
                }
            }
        }

        if paxos_propose.is_none() {
            error!(
                "Paxos propose failed with proposal number = {}",
                proposal_number
            );
            return Err(Status::internal("Paxos propose failed"));
        }

        let paxos_commit = PaxosCommit {
            proposal_number,
            commit: true,
        };

        let mut responses = Vec::new();
        for follower in self.node_state.lock().await.followers {
            let mut client = PaxosServiceClient::connect(follower).await.unwrap();
            let response = client.commit(paxos_commit.clone()).await;
            responses.push(response);
        }

        for response in responses {
            if let Ok(_) = response {
                return Ok(Response::new(JobResponse {
                    job: Some(Job {
                        job_id: job_id as i64,
                        priority,
                        payload,
                    }),
                }));
            }
        }

        error!(
            "Paxos commit failed with proposal_number = {}",
            proposal_number
        );
        return Err(Status::internal("Paxos commit failed"));
    }

    async fn get_task(
        &self,
        request: Request<JobRequest>,
    ) -> Result<Response<JobResponse>, Status> {
        todo!()
    }
}
