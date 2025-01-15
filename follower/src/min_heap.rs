use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::mem;

use uuid::Uuid;

#[derive(Debug)]
pub struct MinHeap {
    pub heap: VecDeque<HeapNode>,
    pub aging_factor: f32,
}

#[derive(PartialEq, Eq)]
pub struct HeapNode {
    /// The unique job id of the job
    pub job_id: Uuid,
    /// The original priority of the job
    pub priority: u32,
    /// Dynamically computed priority to ensure low-priority jobs eventually get processed
    pub effective_priority: u32,
    /// The Lamport timestamp when the job was enqueued onto the
    pub enqueue_time: u64,
}

impl HeapNode {
    pub fn new(job_id: Uuid, priority: u32, timestamp: u64) -> Self {
        return HeapNode {
            job_id,
            priority,
            effective_priority: priority,
            enqueue_time: timestamp,
        };
    }
}

impl Display for HeapNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HeapNode {{\n\tpriority: {}\n\tjob_id: {}\n}}\n",
            self.priority, self.job_id
        )
    }
}

impl fmt::Debug for HeapNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HeapNode {{\n\tpriority: {}\n\tjob_id: {}\n}}\n",
            self.priority, self.job_id
        )
    }
}

impl MinHeap {
    pub fn new(aging_factor: f32) -> Self {
        return MinHeap {
            heap: VecDeque::new(),
            aging_factor,
        };
    }

    /// Inserts a new job into the min heap.
    pub fn insert(&mut self, priority: u32, job_id: Uuid, timestamp: u64) {
        let node: HeapNode = HeapNode::new(job_id, priority, timestamp);
        self.heap.push_back(node);
        println!("Starting at index {}", self.heap.len() - 1);
        self.bubble_up(self.heap.len() - 1);
    }

    fn bubble_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let mut i = index;
        loop {
            let current = match self.heap.get(i) {
                Some(c) => c,
                None => return,
            };

            let parent = match self.get_parent(i) {
                Some(p) => p,
                None => return,
            };

            if &parent.effective_priority > &current.effective_priority {
                if i == 0 {
                    return;
                }
                let parent: usize = ((i - 1) as f64 / 2.0).floor() as usize;
                let mut temp: HeapNode = HeapNode::new(Uuid::new_v4(), 0, 0);
                mem::swap(&mut self.heap[i], &mut temp);
                mem::swap(&mut temp, &mut self.heap[parent]);
                mem::swap(&mut self.heap[i], &mut temp);

                i = parent;
            } else {
                return;
            }
        }
    }

    fn bubble_down(&mut self, index: usize) {
        let mut min: usize = index;

        let current: &HeapNode = match self.heap.get(index) {
            Some(c) => c,
            None => return,
        };

        let left_index: usize = (index * 2) + 1;
        let right_index: usize = (index * 2) + 2;

        let (left_child, right_child): (Option<&HeapNode>, Option<&HeapNode>) =
            self.get_children(index);

        if left_child.is_some()
            && left_child.unwrap().effective_priority < current.effective_priority
        {
            min = left_index;
        }

        if right_child.is_some()
            && right_child.unwrap().effective_priority < current.effective_priority
            && min == index
        {
            min = right_index;
        } else if right_child.is_some()
            && right_child.unwrap().effective_priority < left_child.unwrap().effective_priority
            && min == left_index
        {
            min = right_index;
        }

        if index != min {
            let mut temp: HeapNode = HeapNode::new(Uuid::new_v4(), 0, 0);
            mem::swap(&mut self.heap[index], &mut temp);
            mem::swap(&mut temp, &mut self.heap[min]);
            mem::swap(&mut self.heap[index], &mut temp);

            self.bubble_down(min);
        }
    }

    /// Extracts the top node from the heap.
    pub fn get_top(&mut self) -> Option<HeapNode> {
        if self.heap.is_empty() {
            return None;
        }

        if self.heap.len() == 1 {
            return self.heap.pop_front();
        }

        let mut top: HeapNode = HeapNode::new(Uuid::new_v4(), 0, 0);
        //let last = self.heap.pop_back().unwrap();
        //self.heap.push_front(last);

        let last_index = self.heap.len() - 1;
        mem::swap(&mut self.heap[last_index], &mut top);
        mem::swap(&mut top, &mut self.heap[0]);
        self.heap.pop_back();

        self.bubble_down(0);
        return Some(top);
    }

    /// Retrieves the value of the top node to see if there is a node in the heap.
    pub fn peek(&self) -> Option<&HeapNode> {
        return self.heap.get(0);
    }

    /// Changes the priority of a `HeapNode` in the min heap.
    pub fn change_priority(&mut self, job_id: Uuid, new_priority: u32) -> bool {
        let target_index = match self.heap.iter().position(|n| n.job_id == job_id) {
            Some(i) => i,
            None => return false,
        };

        let target: &mut HeapNode = &mut self.heap[target_index];
        let old_priority = target.priority;

        target.priority = new_priority;
        target.effective_priority = new_priority;

        if old_priority < new_priority {
            self.bubble_up(target_index);
        } else {
            self.bubble_down(target_index);
        }

        return true;
    }

    /// Fetches the child of the node at the given index, if there are any.
    /// Returns a tuple holding references to the children (left_child, right_child)
    fn get_children(&self, current_index: usize) -> (Option<&HeapNode>, Option<&HeapNode>) {
        let left_child: usize = (current_index * 2) as usize + 1;
        let right_child: usize = (current_index * 2) as usize + 2;

        return (self.heap.get(left_child), self.heap.get(right_child));
    }

    /// Returns a reference to the parent node for the child at the provided index.
    fn get_parent(&self, current_index: usize) -> Option<&HeapNode> {
        if current_index == 0 {
            return None;
        }
        let parent: usize = ((current_index - 1) as f64 / 2.0).floor() as usize;
        return self.heap.get(parent);
    }

    pub fn calculate_effective_priority(&mut self, timestamp: u64) {
        for job in self.heap.iter_mut() {
            job.effective_priority =
                job.priority - (self.aging_factor * (timestamp - job.enqueue_time) as f32) as u32;
        }

        self.heapify();
    }

    fn heapify(&mut self) {
        let mut index: usize = 0;
        loop {
            let current = match self.heap.get(index) {
                Some(c) => c,
                None => break,
            };

            let left_idx = 2 * index + 1;
            let right_idx = 2 * index + 2;

            let left_node = self.heap.get(left_idx);
            let right_node = self.heap.get(right_idx);

            let mut min = index;

            if left_node.is_some()
                && left_node.unwrap().effective_priority < current.effective_priority
            {
                min = left_idx;
            }

            if right_node.is_some()
                && right_node.unwrap().effective_priority < current.effective_priority
                || left_node.is_some()
                    && right_node.is_some()
                    && right_node.unwrap().effective_priority
                        < left_node.unwrap().effective_priority
            {
                min = right_idx;
            }

            if min != index {
                //swap
                let mut temp: HeapNode = HeapNode::new(Uuid::new_v4(), 0, 0);
                mem::swap(&mut self.heap[index], &mut temp);
                mem::swap(&mut temp, &mut self.heap[min]);
                mem::swap(&mut self.heap[index], &mut temp);
            }

            index = min;
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::{HeapNode, MinHeap};

    #[test]
    fn test_heap_insert() {
        let mut min_heap: MinHeap = MinHeap::new(0.5);
        // priority,job_id
        let ids: Vec<Uuid> = vec![
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        ];

        min_heap.insert(5, ids[0], 0);
        assert_eq!(min_heap.heap, vec![HeapNode::new(ids[0], 5, 0)]);

        min_heap.insert(3, ids[1], 1);
        assert_eq!(
            min_heap.heap,
            vec![HeapNode::new(ids[1], 3, 1), HeapNode::new(ids[0], 5, 0)]
        );

        min_heap.insert(2, ids[2], 2);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(ids[2], 2, 2),
                HeapNode::new(ids[0], 5, 0),
                HeapNode::new(ids[1], 3, 1)
            ]
        );

        min_heap.insert(1, ids[3], 3);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(ids[3], 1, 3),
                HeapNode::new(ids[2], 2, 2),
                HeapNode::new(ids[1], 3, 1),
                HeapNode::new(ids[0], 5, 0)
            ]
        );

        min_heap.insert(4, ids[4], 4);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(ids[3], 1, 3),
                HeapNode::new(ids[2], 2, 2),
                HeapNode::new(ids[1], 3, 1),
                HeapNode::new(ids[0], 5, 0),
                HeapNode::new(ids[4], 4, 4)
            ]
        );

        min_heap.insert(2, ids[5], 5);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(ids[3], 1, 3),
                HeapNode::new(ids[2], 2, 2),
                HeapNode::new(ids[5], 2, 5),
                HeapNode::new(ids[0], 5, 0),
                HeapNode::new(ids[4], 4, 4),
                HeapNode::new(ids[1], 3, 1),
            ]
        );

        min_heap.insert(2, ids[6], 6);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(ids[3], 1, 3),
                HeapNode::new(ids[2], 2, 2),
                HeapNode::new(ids[5], 2, 5),
                HeapNode::new(ids[0], 5, 0),
                HeapNode::new(ids[4], 4, 4),
                HeapNode::new(ids[1], 3, 1),
                HeapNode::new(ids[6], 2, 6)
            ]
        );

        min_heap.insert(2, ids[7], 7);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(ids[3], 1, 3),
                HeapNode::new(ids[2], 2, 2),
                HeapNode::new(ids[5], 2, 5),
                HeapNode::new(ids[7], 2, 7),
                HeapNode::new(ids[4], 4, 4),
                HeapNode::new(ids[1], 3, 1),
                HeapNode::new(ids[6], 2, 6),
                HeapNode::new(ids[0], 5, 0),
            ]
        );

        min_heap.insert(1, ids[8], 8);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(ids[3], 1, 3),
                HeapNode::new(ids[8], 1, 8),
                HeapNode::new(ids[5], 2, 5),
                HeapNode::new(ids[2], 2, 2),
                HeapNode::new(ids[4], 4, 4),
                HeapNode::new(ids[1], 3, 1),
                HeapNode::new(ids[6], 2, 6),
                HeapNode::new(ids[0], 5, 0),
                HeapNode::new(ids[7], 2, 7),
            ]
        );
    }

    #[test]
    fn test_extraction() {
        let mut min_heap: MinHeap = MinHeap::new(0.5);
        // priority,job_id
        min_heap.insert(5, 1, 0);
        min_heap.insert(3, 2, 1);
        min_heap.insert(2, 3, 2);
        min_heap.insert(1, 4, 3);
        min_heap.insert(4, 5, 4);
        min_heap.insert(2, 6, 5);
        min_heap.insert(2, 7, 6);
        min_heap.insert(2, 8, 7);
        min_heap.insert(1, 9, 8);

        assert_eq!(min_heap.peek().unwrap().priority, 1);
        assert_eq!(min_heap.get_top().unwrap().priority, 1);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(9, 1, 8),
                HeapNode::new(8, 2, 7),
                HeapNode::new(6, 2, 5),
                HeapNode::new(3, 2, 2),
                HeapNode::new(5, 4, 4),
                HeapNode::new(2, 3, 1),
                HeapNode::new(7, 2, 6),
                HeapNode::new(1, 5, 0),
            ]
        );

        assert_eq!(min_heap.peek().unwrap().priority, 1);
        assert_eq!(min_heap.get_top().unwrap().priority, 1);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(8, 2, 7),
                HeapNode::new(3, 2, 2),
                HeapNode::new(6, 2, 5),
                HeapNode::new(1, 5, 0),
                HeapNode::new(5, 4, 4),
                HeapNode::new(2, 3, 1),
                HeapNode::new(7, 2, 6),
            ]
        );

        assert_eq!(min_heap.peek().unwrap().priority, 2);
        assert_eq!(min_heap.get_top().unwrap().priority, 2);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(7, 2, 6),
                HeapNode::new(3, 2, 2),
                HeapNode::new(6, 2, 5),
                HeapNode::new(1, 5, 0),
                HeapNode::new(5, 4, 4),
                HeapNode::new(2, 3, 1),
            ]
        );

        assert_eq!(min_heap.peek().unwrap().priority, 2);
        assert_eq!(min_heap.get_top().unwrap().priority, 2);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(3, 2, 2),
                HeapNode::new(2, 3, 1),
                HeapNode::new(6, 2, 5),
                HeapNode::new(1, 5, 0),
                HeapNode::new(5, 4, 4),
            ]
        );

        assert_eq!(min_heap.peek().unwrap().priority, 2);
        assert_eq!(min_heap.get_top().unwrap().priority, 2);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(6, 2, 5),
                HeapNode::new(2, 3, 1),
                HeapNode::new(5, 4, 4),
                HeapNode::new(1, 5, 0),
            ]
        );

        assert_eq!(min_heap.peek().unwrap().priority, 2);
        assert_eq!(min_heap.get_top().unwrap().priority, 2);
        assert_eq!(
            min_heap.heap,
            vec![
                HeapNode::new(2, 3, 1),
                HeapNode::new(1, 5, 0),
                HeapNode::new(5, 4, 4),
            ]
        );

        assert_eq!(min_heap.peek().unwrap().priority, 3);
        assert_eq!(min_heap.get_top().unwrap().priority, 3);
        assert_eq!(
            min_heap.heap,
            vec![HeapNode::new(5, 4, 4), HeapNode::new(1, 5, 0)]
        );

        assert_eq!(min_heap.peek().unwrap().priority, 4);
        assert_eq!(min_heap.get_top().unwrap().priority, 4);
        assert_eq!(min_heap.heap, vec![HeapNode::new(1, 5, 0),]);

        assert_eq!(min_heap.peek().unwrap().priority, 5);
        assert_eq!(min_heap.get_top().unwrap().priority, 5);
        assert!(min_heap.heap.is_empty());

        assert!(min_heap.peek().is_none());
        assert!(min_heap.get_top().is_none());
        assert!(min_heap.heap.is_empty());
    }
}
