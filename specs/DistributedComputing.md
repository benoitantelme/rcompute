# Distributed Computing Application - Minimal Specification

## 1. Scope

A distributed computing system composed of:

- One **Orchestrator** node.
- Multiple **Worker** nodes.

The orchestrator distributes calculation tasks to workers, collects and merge the results.

It will be ''locally'' distributed first.

---

## 2. Components

### Orchestrator

Responsibilities:

- Maintain the list of known workers.
- Assign tasks to available workers.
- Receive task results.
- Detect worker timeouts and execution failures.
- Reassign failed or timed-out tasks.
- Monitor worker availability.

### Worker

Responsibilities:

- Register its availability with the orchestrator.
- Receive tasks from the orchestrator.
- Execute assigned tasks.
- Return either:
  - A successful result.
  - An execution error.

---

## 3. Worker Discovery

- All workers must be configured with the orchestrator network reference (IP address or hostname, this will be the orchestrator self reference locally).
- Workers initiate communication with the orchestrator.
- The orchestrator maintains the current worker pool.

---

## 4. Task Lifecycle

### Assignment

1. A task enters the orchestrator queue.
2. The orchestrator selects a free worker.
3. The task is assigned to the worker.

### Success

1. The worker completes the task.
2. The worker returns the result.
3. The orchestrator marks the task as completed.

### Failure

1. The worker returns an execution error.
2. The orchestrator marks the assignment as failed.
3. The task is reassigned to another free worker.

### Timeout

1. A configurable timeout is associated with each task assignment.
2. If no result is received before the timeout expires:
   - The assignment is considered failed.
   - The worker is removed from the active worker pool.
   - The task is reassigned to another free worker.

### Late Result

- If a timed-out worker later returns a result:
  - The worker is returned to the active worker pool.
  - The returned result is ignored.

---

## 5. Worker Pool Management

### Workers for task

Workers will be created for a task in order to not have idle workers:

- So if we don't have work, we don't have workers consuming resources for nothing.
- We scale when needed.
- If we end-up needing workers we don't have it's a planification problem and we should panic


### Timed-out Worker

A worker can time out when it takes too long to execute its task:

- It needs to be detected.
- It needs to be de activated.
- It needs to be replaced by a new worker for the task.
- Need to have a panic detection pattern in the future, maybe canceling a calculation if one task is failing or timing out repeatedly.

We use a channel and a binary heap with preset timeouts to check if we have timed out workers. If we find a timed out worker is still supposed to be in use, we can clean it up.

---

## 6. Task Reassignment Rules

- Failed tasks must be reassigned to a different active worker.
- Reassignment occurs after:
  - Execution error.
  - Timeout.
- If no active worker is available:
  - The task remains queued.
  - Assignment is attempted when a worker becomes available.

---

## 7. Resource Monitoring

A configurable parameter defines:

- `threshold`

The orchestrator continuously compares the number of active and available workers to this threshold.

When:

```text
activeWorkers < minimumActiveWorkers
availableWorkers < minimumActiveWorkers
```

the orchestrator must display:

```text
Insufficient available resources.
```

A message is displayed when the available workers count meets or exceeds the threshold.

---

## 8. Configuration

The system must support the following configuration parameters:

- Orchestrator address (IP or hostname, will be a reference for the local version)
- Task timeout
- Minimum available workers threshold
- Initial workers count