# distributed-priority-queue

A distributed priority queue implementation It aims to provide an efficient, fault-tolerant, and scalable system for managing prioritized jobs in a distributed environment. System supports advanced features such as the leader-follower pattern with Paxos, Round-Robin algorithm working with wieighted priority and long polling to pull jobs from distributed nodes.


## Features
### 1. Distributed System Replication
The system is built around a leader-follower pattern comprising of a single leader and 2 followers with a quorum number of 1.
This pattern ensures:
-
-
-


### 2. Task Distribution Strategy

Inspired by Kafka's design, tasks are enqueued into priority buckets. 
* **Round-Robin Algorithm** pulls jobs from the buckets and evenly distributes them to nodes.
* **Weighted priority** assignes weights to priority buckets to constol the share of jobs pulled from each bucket.
    - Priority 1: 30% of scheduling time
    - Priority 2: 25% of scheduling time
    - Priority 3: 20% of scheduling time
    - Priority 4: 15% of scheduling time
    - Priority 5: 10% of scheduling time

### 3. PostgreSQL Integration
The system uses PostgreSQL as the persistent database for storing jobs.
The schema for the table is:
```sql
CREATE TABLE jobs (
    job_id BIGSERIAL PRIMARY KEY,                   -- Auto-incrementing job_id (PRIMARY KEY automatically creates an index)
    priority INT CHECK (priority BETWEEN 1 AND 5),  -- Validate priority (1-5)
    payload BYTEA,                                  -- Payload as a byte array 
    created_at TIMESTAMPTZ DEFAULT now(),           -- Timestamp for creation
);
```

### 4. Logging and Monitoring
The system provides comprehensive logging accross all subsytems and nodes.
* **Error logs** capture and report any errors that occur with relavent details.
* **Request logs** capture details about requests recieved for the given node.

#### Example Log Entries:
Error Log:
```rust
2024-12-27 12:34:56.789 [ERROR] <error_route>: <error_message>
```
Request Log:
```rust
2024-12-27 12:45:00.123 [INFO] index: Received <request_method> request for <request_uri> from IP: <client_IP>
```

### 5. Lamport Timestamps
## Acknowledgements

 - [FOQS Distributed Priority Queue](https://engineering.fb.com/2021/02/22/production-engineering/foqs-scaling-a-distributed-priority-queue/)
 - [Distributed Priority Queue Design](https://www.youtube.com/watch?v=PFsjVT-XwmA) 
 - [Prevent starvation in priority queue](https://stackoverflow.com/questions/6393135/how-to-prevent-low-priority-messages-on-an-activemq-prioritized-queue-from-being)


### Roadmap

1. Enqueue API, Insert Buffer and Load Balancer
    - logging for errors and requests
    - buffer implementation
    - load balancer implementation (round robin and weighted priority)

2. Node (Priority queue,dequeue API and Database)
    

3. Consumer logic for pulling jobs
    - continuously fetch jobs from nodes
    - process jobs and report results
    - implement long polling mechanism for job retrieval
    - process jobs in parallel to showcase concurrency
    - implement retry mechanism?

---
## Tech Stack

* **Languages:** Rust (with tokio runtime)
* **Databse:** PostgreSQL


