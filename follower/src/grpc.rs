use crate::job_management::paxos_service_server::PaxosService;
use crate::job_management::{Job, PaxosAccept, PaxosAck, PaxosPrepare, PaxosPromise};
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
    // The accepted job
    pub accepted_value: Option<Job>,
    // Local min heap
    pub queue: MinHeap,
    // Lamport timestamp
    pub lamport_timestamp: u64,
}

impl PaxosState {
    pub fn new() -> Result<Self, String> {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let _: u64 = match std::env::args().collect::<Vec<String>>().get(1) {
            Some(id) => match id.parse::<u64>() {
                Ok(i) => i,
                Err(_) => {
                    error!(target:"error_logger","Failed to parse node id: Node id must be of type u64");
                    return Err("Failed to parse node id into a u64".to_string());
                }
            },
            None => {
                error!(target:"error_logger","No node id provided in command line arguments: could not start node");
                return Err("No node id provided in command line arguments".to_string());
            }
        };

        Ok(PaxosState {
            promised_proposal: 0,
            accepted_proposal: 0,
            accepted_value: None,
            queue: MinHeap::new(0.5),
            lamport_timestamp: 0,
        })
    }

    pub fn increment_time(&mut self) -> u64 {
        let temp = self.lamport_timestamp;
        self.lamport_timestamp += 1;
        temp
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
        info!(target:"request_logger","Paxos Prepare recieved with proposal number {}",prepare.proposal_number);

        if prepare.proposal_number > state.accepted_proposal {
            state.accepted_proposal = prepare.proposal_number;
            Ok(Response::new(PaxosPromise {
                proposal_number: prepare.proposal_number,
                highest_proposal: state.accepted_proposal,
                promise: state.accepted_proposal == prepare.proposal_number,
            }))
        } else {
            error!(target:"error_logger","Failed Paxos proposal: number was less than promised");
            Err(Status::failed_precondition(
                "Proposal number is less than promised.",
            ))
        }
    }

    /// Recieves the Accpet message from the proposer so that this acceptor can accept the value
    /// and record it.
    ///
    /// # Arguments
    /// `request`: The Paxos Accept message from the proposer.
    ///
    /// # Return
    /// A Result object that is either an Ok(tonic::Response) or Err(tonic::Status)
    async fn accept(&self, request: Request<PaxosAccept>) -> Result<Response<PaxosAck>, Status> {
        let mut state = self.state.lock().await;
        let propose = request.into_inner();
        info!(target:"error_logger","Paxos Accept message recieved with proposal number {}",propose.proposal_number);
        if propose.proposal_number >= state.promised_proposal {
            state.accepted_proposal = propose.proposal_number;
            state.accepted_value = match propose.proposed_job {
                Some(job) => Some(job.clone()),
                None => {
                    error!(target: "error_logger","Failed Accept: no job provided in accept message");
                    return Err(Status::internal("No job provided in accept message"));
                }
            };
            Ok(Response::new(PaxosAck {
                proposal_number: propose.proposal_number,
            }))
        } else {
            error!(target:"error_logger","Failed Paxos Accept: proposal number was less than promised");
            Err(Status::failed_precondition(
                "Proposal number is less than promised.",
            ))
        }
    }
}
