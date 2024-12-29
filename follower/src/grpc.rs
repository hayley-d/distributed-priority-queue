use crate::job_management::paxos_service_server::PaxosService;
use crate::job_management::{
    Job, PaxosAccept, PaxosCommit, PaxosCommitResponse, PaxosPrepare, PaxosPromise, PaxosPropose,
};
use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct PaxosState {
    pub promised_proposal: i32,
    pub accepted_proposal: i32,
    pub accepted_value: Option<Job>,
}

impl PaxosState {
    pub fn new() -> Self {
        PaxosState {
            promised_proposal: 0,
            accepted_proposal: 0,
            accepted_value: None,
        }
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

        if commit.commit {
            if let Some(_) = &state.accepted_value {
                state.accepted_value = None;
                Ok(Response::new(PaxosCommitResponse {
                    proposal_number: commit.proposal_number,
                }))
            } else {
                error!("Failed Paxos commit: no accpeted value to commit");
                Err(Status::failed_precondition("No accepted value to commit."))
            }
        } else {
            error!("Failed Paxos commit: commit falg is false");
            Err(Status::failed_precondition("Commit flag is false."))
        }
    }
}
