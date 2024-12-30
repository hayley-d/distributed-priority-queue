mod load_balancer {
    use std::collections::VecDeque;

    use log::{error, info};

    use crate::job_management::node_health_service_client::NodeHealthServiceClient;
    use crate::job_management::{EnqueueRequest, NodeHealthRequest};

    pub struct Node {
        address: String,
        weight: f32,
    }

    impl Eq for Node {}

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.weight == other.weight && self.address == other.address
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.weight < other.weight {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    }

    impl Node {
        pub fn new(address: String, weight: f32) -> Self {
            return Node { address, weight };
        }
    }

    pub struct LoadBalancer {
        buffer: VecDeque<EnqueueRequest>,
        nodes: Vec<Node>,
        available_nodes: u32,
    }

    impl LoadBalancer {
        pub fn new(available_nodes: u32, addresses: Vec<String>) -> Self {
            let mut nodes: Vec<Node> = Vec::with_capacity(available_nodes as usize);

            for address in addresses {
                let weight: f32 = Self::get_weight(&address);
                let node: Node = Node::new(address, weight);
                nodes.push(node);
            }

            nodes.sort_by(|a, b| b.cmp(&a));

            return LoadBalancer {
                buffer: VecDeque::new(),
                nodes,
                available_nodes,
            };
        }

        /// Calculates the weight of the leader
        async fn get_weight(address: &String) -> f32 {
            let request: NodeHealthRequest = NodeHealthRequest {};

            // log gRPC request
            info!("NodeHealthService Request to address {}", address);

            let mut client = NodeHealthServiceClient::connect(address.into())
                .await
                .unwrap();
            let response = client.get_node_health(request.clone()).await;

            match response {
                Ok(res) => {
                    let res = res.into_inner().clone();
                    let (cpu_utilization, memory_usage, queue_depth, response_time) = (
                        &res.cpu_utilization,
                        &res.memory_usage,
                        &res.queue_depth,
                        &res.response_time,
                    );

                    let weight: f32 = (((1.0 - cpu_utilization) / 100.0)
                        * ((1.0 - memory_usage) / 100.0)
                        * ((1.0 - (*queue_depth as f32)) / 100.0))
                        .round()
                        / 100.0;
                }
                Err(_) => {
                    error!(
                        "Failed to obtain node health status from node at {}",
                        address
                    );
                }
            }

            todo!()
        }
    }
}
