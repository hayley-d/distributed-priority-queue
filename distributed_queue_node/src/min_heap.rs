pub struct MinHeap {
    heap: Vec<HeapNode>,
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
        return MinHeap { heap: Vec::new() };
    }

    /// Inserts a new job into the min heap.
    pub async fn insert(priority: u32, job_id: u64) -> bool {
        todo!()
    }

    /// Extracts the top node from the heap.
    pub async fn get_top() -> Option<HeapNode> {
        todo!()
    }

    /// Retrieves the value of the top node to see if there is a node in the heap.
    pub async fn peek() -> Option<u64> {
        todo!()
    }

    /// Changes the priority of a `HeapNode` in the min heap.
    pub async fn change_priority(job_id: u64, new_priority: u32) -> bool {
        todo!()
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
