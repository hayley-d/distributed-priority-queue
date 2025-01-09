<p><a target="_blank" href="https://app.eraser.io/workspace/ABZxvFD0Ln1sSE9MLZsF" id="edit-in-eraser-github-link"><img alt="Edit in Eraser" src="https://firebasestorage.googleapis.com/v0/b/second-petal-295822.appspot.com/o/images%2Fgithub%2FOpen%20in%20Eraser.svg?alt=media&amp;token=968381c8-a7e7-472a-8ed6-4a6626da5501"></a></p>

# Distributed Priority Queue
A scalable, distributed priority queue system designed to efficiently handle high-concurrency workloads. This project highlights key concepts in distributed systems, fault tolerance, and load balancing, drawing inspiration from real-world applications like Facebook's priority queue.

![Figure 2](/.eraser/ABZxvFD0Ln1sSE9MLZsF___XkJZjuhCcuhY39UPh3qdtmdsVUw1___---figure---A4DQbPsdVhKnyiNOqOk7g---figure---_7O9b58eNT6VJfolPZFmaw.png "Figure 2")



## Features
### Protocol Buffers and gRPC
- Utilises **Protocol Buffers (Protobuf)** for efficient and language-agnostic serialization of messages.
- **gRPC** is used for communication between services, ensuring fast and reliable bi-directional streaming.
### Paxos for Leader-Follower Fault Tolerance
- **Paxos** ensures fault tolerance within the leader-follower replication model, maintaining consistency and availability even if nodes or leaders fail.
![Figure 1](/.eraser/ABZxvFD0Ln1sSE9MLZsF___XkJZjuhCcuhY39UPh3qdtmdsVUw1___---figure---9ZzjRp9-F6Uc6Crsbvsw7---figure---2elV4sFTULS_wPqIk2bing.png "Figure 1")

### Leader-Follower Pattern with Quorum-Based Replication
- **Leader-Follower Model**: A leader node handles write operations, while follower nodes replicate data for fault tolerance.
- **Quorum-Based Approach**: Requires a majority of nodes to acknowledge a change before it’s considered committed, ensuring data consistency.
### Dynamic Load Balancer
- A **gRPC-based weighted round-robin load balancer** that dynamically adjusts node weights using real-time health metrics such as CPU usage, queue depth, and task processing rates.
![Figure 3](/.eraser/ABZxvFD0Ln1sSE9MLZsF___XkJZjuhCcuhY39UPh3qdtmdsVUw1___---figure---zWfYm6V9X6nD0Op-qYnJ2---figure---Z18o7S9kzxN3q0VQ_1WuVA.png "Figure 3")

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

## References
**Facebook Engineering** (2021) _FOQS scaling a distributed priority queue_. Available at: [﻿https://engineering.fb.com/2021/02/22/production-engineering/foqs-scaling-a-distributed-priority-queue/](https://engineering.fb.com/2021/02/22/production-engineering/foqs-scaling-a-distributed-priority-queue/) (Accessed: 15 December 2024).

**Jordan has no life** (2020) 31: Distributed Priority Queue | Systems Design Interview Questions With Ex-Google SWE [YouTube]. Available at: [﻿https://www.youtube.com/watch?v=PFsjVT-XwmA](https://www.youtube.com/watch?v=PFsjVT-XwmA) (Accessed: 15 December 2024).

**Stack Overflow** (2011) _How to prevent low-priority messages on an ActiveMQ prioritized queue from being consumed indefinitely?_. Available at: [﻿https://stackoverflow.com/questions/6393135/how-to-prevent-low-priority-messages-on-an-activemq-prioritized-queue-from-being](https://stackoverflow.com/questions/6393135/how-to-prevent-low-priority-messages-on-an-activemq-prioritized-queue-from-being) (Accessed: 20 December 2024).

**Guru99** (2021) _Round-robin scheduling example_. Available at: [﻿https://www.guru99.com/round-robin-scheduling-example.html](https://www.guru99.com/round-robin-scheduling-example.html) (Accessed: 20 December 2024).

**Towards Data Science** (2020) _Priority queues_. Available at: [﻿https://towardsdatascience.com/course-2-data-structure-part-2-priority-queues-and-disjoint-set-ed11a0383011](https://towardsdatascience.com/course-2-data-structure-part-2-priority-queues-and-disjoint-set-ed11a0383011) (Accessed: 20 December 2024).

**Educative** (n.d.) _Leader and follower replication_. Available at: [﻿https://www.educative.io/answers/leader-and-follower-replication](https://www.educative.io/answers/leader-and-follower-replication) (Accessed: 21 December 2024).

**Educative** (n.d.) _What is quorum in distributed systems_. Available at: [﻿https://www.educative.io/answers/what-is-quorum-in-distributed-systems](https://www.educative.io/answers/what-is-quorum-in-distributed-systems) (Accessed: 21 December 2024).





<!--- Eraser file: https://app.eraser.io/workspace/ABZxvFD0Ln1sSE9MLZsF --->