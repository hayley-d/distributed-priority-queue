use crate::job_management::paxos_service_server::PaxosService;
use crate::job_management::{
    Job, PaxosAccept, PaxosCommit, PaxosCommitResponse, PaxosPrepare, PaxosPromise, PaxosPropose,
};
use crate::min_heap::MinHeap;
use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

/// The current Paxos state
#[derive(Debug)]
pub struct PaxosState {
    // The promise number
    pub promised_proposal: i32,
    // The last accepted proposal
    pub accepted_proposal: i32,
    pub accepted_value: Option<Job>,
    pub queue: MinHeap,
    pub lamport_timestamp: u64,
}

impl PaxosState {
    pub fn new() -> Self {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let node_id: u64 = match std::env::args().collect::<Vec<String>>().get(1) {
            Some(id) => match id.parse::<u64>() {
                Ok(i) => i,
                Err(_) => {
                    error!("Failed to parse node id: Node id must be of type u64");
                    std::process::exit(1);
                }
            },
            None => {
                error!("No node id provided in command line arguments: could not start node");
                std::process::exit(1);
            }
        };
        PaxosState {
            promised_proposal: 0,
            accepted_proposal: 0,
            accepted_value: None,
            queue: MinHeap::new(0.5),
            lamport_timestamp: 0,
        }
    }

    pub fn increment_time(&mut self) -> u64 {
        let temp = self.lamport_timestamp;
        self.lamport_timestamp += 1;
        return temp;
    }
}

#[derive(Debug)]
pub struct LocalPaxosService {
    pub state: Arc<Mutex<PaxosState>>,
}

#[tonic::async_trait]
impl PaxosService for LocalPaxosService {
    async fn prepare(
        &self,
        request: Request<PaxosPrepare>,
    ) -> Result<Response<PaxosPromise>, Status> {
        let mut state = self.state.lock().await;
        let prepare = request.into_inner();
        info!(
            "Paxos Prepare recieved with proposal number {}",
            prepare.proposal_number
        );

        if prepare.proposal_number >= state.promised_proposal {
            state.promised_proposal = prepare.proposal_number;
            Ok(Response::new(PaxosPromise {
                proposal_number: prepare.proposal_number,
                accepted_value: state
                    .accepted_value
                    .as_ref()
                    .map(|job| job.priority)
                    .unwrap_or(0),
            }))
        } else {
            error!("Failed Paxos proposal: number was less than promised");
            Err(Status::failed_precondition(
                "Proposal number is less than promised.",
            ))
        }
    }

    async fn propose(
        &self,
        request: Request<PaxosPropose>,
    ) -> Result<Response<PaxosAccept>, Status> {
        let mut state = self.state.lock().await;
        let propose = request.into_inner();
        info!(
            "Paxos Proposal recieved with proposal number {}",
            propose.proposal_number
        );

        if propose.proposal_number >= state.promised_proposal {
            state.accepted_proposal = propose.proposal_number;
            state.accepted_value = match propose.proposed_job {
                Some(job) => Some(job.clone()),
                None => {
                    error!("Failed proposal: no job provided in proposal");
                    return Err(Status::internal("No job provided in proposal"));
                }
            };

            Ok(Response::new(PaxosAccept {
                proposal_number: propose.proposal_number,
                accepted: true,
            }))
        } else {
            error!("Failed Paxos proposal: proposal number was less than promised");
            Err(Status::failed_precondition(
                "Proposal number is less than promised.",
            ))
        }
    }

    async fn commit(
        &self,
        request: Request<PaxosCommit>,
    ) -> Result<Response<PaxosCommitResponse>, Status> {
        let mut state = self.state.lock().await;
        let commit = &request.into_inner();
        info!(
            "Paxos Commit recieved with proposal number {} and status {}",
            commit.proposal_number,
            match commit.commit {
                true => "success",
                _ => "failed",
            }
        );

        let time = state.increment_time();
        if commit.commit {
            let job: Option<Job> = state.accepted_value.clone();
            match job {
                Some(job) => {
                    state
                        .queue
                        .insert(job.priority as u32, job.job_id as u64, time);
                    state.accepted_value = None;

                    Ok(Response::new(PaxosCommitResponse {
                        proposal_number: commit.proposal_number,
                    }))
                }
                None => {
                    error!("Failed Paxos commit: no accpeted value to commit");
                    Err(Status::failed_precondition("No accepted value to commit."))
                }
            }
        } else {
            error!("Failed Paxos commit: commit falg is false");
            Err(Status::failed_precondition("Commit flag is false."))
        }
    }
}
