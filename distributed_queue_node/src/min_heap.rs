use std::collections::VecDeque;
use std::mem;

pub struct MinHeap {
    heap: VecDeque<HeapNode>,
}

pub struct HeapNode {
    pub priority: u32,
    pub job_id: u64,
}

impl HeapNode {
    pub fn new(priority: u32, job_id: u64) -> Self {
        return HeapNode { priority, job_id };
    }
}
impl MinHeap {
    pub fn new() -> Self {
        return MinHeap {
            heap: VecDeque::new(),
        };
    }

    /// Inserts a new job into the min heap.
    pub fn insert(&mut self, priority: u32, job_id: u64) {
        let node: HeapNode = HeapNode::new(priority, job_id);
        self.heap.push_back(node);
        self.bubble_up(self.heap.len() - 1);
    }

    fn heapify(&mut self, current_index: usize) {
        let current: &HeapNode = match self.heap.get(current_index) {
            Some(node) => node,
            None => return,
        };
        let left_index: usize = (current_index * 2) as usize + 1;
        let right_index: usize = (current_index * 2) as usize + 2;

        let (left_child, right_child): (Option<&HeapNode>, Option<&HeapNode>) =
            self.get_children(current_index);

        let mut smallest: usize = current_index;

        if left_child.is_some() && left_child.unwrap().priority < current.priority {
            smallest = left_index;
        }

        if right_child.is_some() && right_child.unwrap().priority < current.priority {
            smallest = right_index;
        }

        if smallest != current_index {
            let mut temp: HeapNode = HeapNode::new(0, 0);
            mem::swap(&mut self.heap[current_index], &mut temp);
            mem::swap(&mut temp, &mut self.heap[smallest]);
            self.heapify(smallest);
        }

        return;
    }

    fn bubble_up(&mut self, index: usize) {
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
            if &parent.priority > &current.priority {
                let parent: usize = ((i - 1) as f64 / 2.0).floor() as usize;
                let mut temp: HeapNode = HeapNode::new(0, 0);
                mem::swap(&mut self.heap[i], &mut temp);
                mem::swap(&mut temp, &mut self.heap[parent]);

                i = parent;
            }
        }
    }

    fn bubble_down(&mut self, index: usize) {
        let mut max: usize = index;

        let current: &HeapNode = match self.heap.get(index) {
            Some(c) => c,
            None => return,
        };

        let left_index: usize = (index * 2) as usize + 1;
        let right_index: usize = (index * 2) as usize + 2;
        let (left_child, right_child): (Option<&HeapNode>, Option<&HeapNode>) =
            self.get_children(index);

        if left_child.is_some() && left_child.unwrap().priority < current.priority {
            max = left_index;
        }

        if right_child.is_some() && right_child.unwrap().priority < current.priority {
            max = right_index;
        }

        if index != max {
            let mut temp: HeapNode = HeapNode::new(0, 0);
            mem::swap(&mut self.heap[index], &mut temp);
            mem::swap(&mut temp, &mut self.heap[max]);
            return self.bubble_down(max);
        }
    }

    /// Extracts the top node from the heap.
    pub async fn get_top(&mut self) -> Option<HeapNode> {
        return self.heap.pop_front();
    }

    /// Retrieves the value of the top node to see if there is a node in the heap.
    pub async fn peek(&self) -> Option<&HeapNode> {
        return self.heap.get(0);
    }

    /// Changes the priority of a `HeapNode` in the min heap.
    pub async fn change_priority(&mut self, job_id: u64, new_priority: u32) -> bool {
        let target_index = match self.heap.iter().position(|n| n.job_id == job_id) {
            Some(i) => i,
            None => return false,
        };

        let target: &mut HeapNode = &mut self.heap[target_index];
        let old_priority = target.priority;

        target.priority = new_priority;

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
        let parent: usize = ((current_index - 1) as f64 / 2.0).floor() as usize;
        return self.heap.get(parent);
    }
}
