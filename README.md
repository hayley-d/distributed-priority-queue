# distributed-priority-queue

A distributed priority queue implementation in Rust.

### TODOs
1. Enqueue API, Insert Buffer and Load Balancer
    - Enqueue API provides a public facing interface for clients to submit jobs into the queue
    - Insert buffer acts as a temporary holding area, optimizing batch inserts
    - Load balanacer decides which node should handle a job ensuring even distribution accross nodes. (Round Robin distribution)

2. Node (Priority queue,dequeue API and Database)
    - Priority queue: min heap
    - Dequeue API: allows workers to pull jobs from the node
    - database: Centralized postgreSQL

3. Consumer logic for pulling jobs
    - continuously fetch jobs from nodes
    - process jobs and report results
    - implement long polling mechanism for job retrieval
    - process jobs in parallel to showcase concurrency
    - implement retry mechanism?
