# Important rules

## 1. A server grants its vote only if the candidate's log is at least as up-to-date as its own

The voter denies its vote if its own log is more up-to-date than that of the candidate.

Raft determines which of two logs is more up-to-date by comparing the index and term of the last entries in the logs. If the logs have last entries with different terms, then the log with the later term is more up-to-date. If the logs end with the same term, then whichever log is longer is more up-to-date.

## 2. A leader commits entries by counting replicas only for entries from its current term

To eliminate problems like the one in Figure 8, Raft never commits log entries from previous terms by counting replicas. Only log entries from the leader's current term are committed by counting replicas.

# RSM may wait indefinitely after submitting a request to Raft

This can occur even if the request has already been replicated to a majority.

```text

  Node 0            Node 1            Node 2            Node 3            Node 4
┌────────┐        ┌────────┐        ┌────────┐        ┌────────┐        ┌────────┐
│  Raft  │ <----> │  Raft  │ <----> │  Raft  │ <----> │  Raft  │ <----> │  Raft  │
├──-──-──┤        ├──-──-──┤        ├──-──-──┤        ├──-──-──┤        ├──-──-──┤
│  RSM   │        │  RSM   │        │  RSM   │        │  RSM   │        │  RSM   │
├──-──-──┤        ├──-──-──┤        ├──-──-──┤        ├──-──-──┤        ├──-──-──┤
│KVServer│        │KVServer│        │KVServer│        │KVServer│        │KVServer│
└────────┘        └────────┘        └────────┘        └────────┘        └────────┘



                ┌─────┐
                │Clerk│
                └─────┘

```

Assume Raft_0 is elected leader in term_1. A Clerk sends a command to KVServer_0 (--> RSM_0 --> Raft_0). After that, **no further commands are submitted by any client**.

- If this command has been replicated to the minority, it's not guaranteed to be committed, RSM_0 can wait forever.
- If this command has been replicated to the majority, it's guaranteed to be committed, but RSM_0 can still wait forever.

## Explanation

Consider the following scenario:

Raft_0 successfully replicates the `term_1/command` log entry to a majority, but becomes partitioned before receiving the `AppendEntries` replies. As a result, the entry remains uncommitted, even though it exists on a majority of servers and is guaranteed to be applied sometimes in the future.

```text
Raft_0.logs = [term_0/nil, term_1/command]
Raft_1.logs = [term_0/nil, term_1/command]
Raft_2.logs = [term_0/nil, term_1/command]
Raft_3.logs = [term_0/nil]
Raft_4.logs = [term_0/nil]
```

A network partition occurs:

- `p1 = [0]`
- `p2 = [1,2,3,4]`

Within `p2`, a new leader must be elected. Since Raft_1 and Raft_2 have more up-to-date logs, the next leader can only be one of them. Suppose Raft_1 wins the election and replicates its log to Raft_3 and Raft_4.

```text
Raft_0.logs = [term_0/nil, term_1/command]
Raft_1.logs = [term_0/nil, term_1/command]
Raft_2.logs = [term_0/nil, term_1/command]
Raft_3.logs = [term_0/nil, term_1/command]
Raft_4.logs = [term_0/nil, term_1/command]
```

At this point, `term_1/command` is stored on all servers in p2. However, Raft_1 cannot commit this entry because it was created in a previous term. According to the Raft paper (Figure 8), a leader may only advance commitIndex for entries from the current term by counting replicas.

When Raft_0 later reconnects, it steps down and follows Raft_1. Nevertheless, `term_1/command` remains uncommitted unless a new command from the current term is appended and replicated to a majority.

Consequently, RSM_0 may wait indefinitely for `term_1/command` to be applied if no new client request arrives to trigger commitment.

## Solution: periodically submit a "No-Op" command from all RSM instances

Each RSM instance periodically submits a no-op command to its local Raft node.

The interval can be relatively long (e.g., 1 second), since this scenario is rare under normal traffic. However, introducing a periodic no-op ensures that a command will not remain uncommitted indefinitely even if it has already been replicated to all Raft instances.

## Solution: submit a "No-Op" command while elected as a leader

> First, a leader must have the latest information on which entries are committed. The Leader Completeness Property guarantees that a leader has all committed entries, but at the start of its term, it may not know which those are. To find out, it needs to commit an entry from its term. Raft handles this by having each leader commit a blank no-op entry into the log at the start of its term.

Read the paper!
