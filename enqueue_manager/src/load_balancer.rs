mod load_balancer {
    use std::collections::VecDeque;

    use crate::job_management::EnqueueRequest;

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
        fn get_weight(address: &String) -> f32 {
            todo!()
        }
    }
}
