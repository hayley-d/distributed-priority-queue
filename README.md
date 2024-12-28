<p><a target="_blank" href="https://app.eraser.io/workspace/PgkXybzAAX4Qs9me1TaF" id="edit-in-eraser-github-link"><img alt="Edit in Eraser" src="https://firebasestorage.googleapis.com/v0/b/second-petal-295822.appspot.com/o/images%2Fgithub%2FOpen%20in%20Eraser.svg?alt=media&amp;token=968381c8-a7e7-472a-8ed6-4a6626da5501"></a></p>

# Distributed Priority Queue
A scalable, distributed priority queue system designed to efficiently handle high-concurrency workloads. This project highlights key concepts in distributed systems, fault tolerance, and load balancing, drawing inspiration from real-world applications like Facebook's priority queue.

![Figure 1](undefined "Figure 1")

## Features
### Protocol Buffers and gRPC
- Utilises **Protocol Buffers (Protobuf)** for efficient and language-agnostic serialization of messages.
- **gRPC** is used for communication between services, ensuring fast and reliable bi-directional streaming.
### Paxos for Leader-Follower Fault Tolerance
- **Paxos** ensures fault tolerance within the leader-follower replication model, maintaining consistency and availability even if nodes or leaders fail.
### Leader-Follower Pattern with Quorum-Based Replication
- **Leader-Follower Model**: A leader node handles write operations, while follower nodes replicate data for fault tolerance.
- **Quorum-Based Approach**: Requires a majority of nodes to acknowledge a change before it’s considered committed, ensuring data consistency.
### Round-Robin Load Balancer with Weighted Priorities
- A **Round-Robin Load Balancer** distributes tasks evenly across nodes while respecting job priority.
- **Weighted Priorities** ensure that tasks are scheduled according to their importance:
    - Priority 1: 30% of scheduling time
    - Priority 2: 25% of scheduling time
    - Priority 3: 20% of scheduling time
    - Priority 4: 15% of scheduling time
    - Priority 5: 10% of scheduling time
### Long Polling with Pull Model for Consumers
- **Long Polling** allows consumers to pull jobs from the queue only when they are available, optimizing resource usage and reducing idle time.
- **Pull Model** ensures that consumers only retrieve jobs when needed, improving overall efficiency.
### PostgreSQL Integration
The system uses PostgreSQL for persistent job storage. 

The schema for the job table:

```
CREATE TABLE jobs (
    job_id BIGSERIAL PRIMARY KEY,                   -- Auto-incrementing job_id
    priority INT CHECK (priority BETWEEN 1 AND 5),  -- Validate priority (1-5)
    payload BYTEA,                                  -- Payload as a byte array
    created_at TIMESTAMPTZ DEFAULT now(),           -- Timestamp for creation
);
```
### Logging
- **Error Logs**: Tracks and reports errors with relevant details.
- **Request Logs**: Logs details about requests received by each node.
**Example log entries:**

Error Log:

```
2024-12-27 12:34:56.789 [ERROR] <error_route>: <error_message>
```
Request Log: 

```
2024-12-27 12:45:00.123 [INFO] index: Received <request_method> request for <request_uri> from IP: <client_IP>
```
## System Architecture
The system consists of four main components, each with distinct responsibilities:

### 1. **Enqueue Manager**
- **Role**: Provides an API for job submission, buffering incoming tasks, and distributing them to nodes.
- **Functionality**:
    - Buffers tasks temporarily before distributing them based on priority and node availability.
    - The **Round-Robin Load Balancer** distributes tasks according to weighted priority, ensuring high-priority tasks are prioritized but maintaining fairness across all nodes.
### 2. **Leader**
- **Role**: The leader node processes job enqueue requests and manages data replication across followers.
- **Functionality**:
    - Receives job submission requests from the enqueue manager.
    - Inserts new jobs into the database after the Paxos consensus protocol ensures data consistency.
    - Maintains fault tolerance by managing leader-follower replication.
### 3. **Follower**
- **Role**: The follower nodes contain local priority queues and implement long-polling mechanisms for pulling jobs.
- **Functionality**:
    - Each follower node maintains a local queue and processes jobs based on their priority.
    - Long-polling ensures that a consumer only retrieves jobs when they are available, optimizing system resources and reducing idle time.
### 4. **Consumer**
- **Role**: The consumer pulls jobs from the distributed queue, processes them, and acknowledges their completion.
- **Functionality**:
    - Uses long-polling to efficiently wait for and fetch jobs, ensuring resources are not wasted while waiting for tasks.
![Figure 2](undefined "Figure 2")





<!--- Eraser file: https://app.eraser.io/workspace/PgkXybzAAX4Qs9me1TaF --->