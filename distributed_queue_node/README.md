
## Database Schema
The database contains the following table:
```sql
CREATE TABLE jobs (
    job_id BIGSERIAL PRIMARY KEY,         -- Auto-incrementing job_id (PRIMARY KEY automatically creates an index)
    priority INT CHECK (priority BETWEEN 1 AND 20), -- Validate priority (1-20)
    payload BYTEA,                        -- Payload as a byte array 
    created_at TIMESTAMPTZ DEFAULT now(), -- Timestamp for creation
);
```


