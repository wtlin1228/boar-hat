package raft

// startAt=0, data=[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// - getCount() // 10
// - getLastLogIndex() // 10
// - getLogEntry(5) // 5
// - replace(5, [15, 16, 17]) // data=[0, 1, 2, 3, 4, 15, 16, 17]
// - trim(5) // start=5, data=[5, 6, 7, 8, 9, 10]
// startAt=3, data=[3, 4, 5, 6, 7, 8, 9, 10]
// - getCount() // 10
// - getLastLogIndex() // 10
// - getLogEntry(5) // 5
// - replace(5, [15, 16, 17]) // data=[3, 4, 15, 16, 17]
// - trim(5) // start=5, data=[5, 6, 7, 8, 9, 10]

import "slices"

type LogEntry struct {
	Term    int
	Command any
}

// Raft log is 1-indexed, and will be trimmed after snapshot to prevent log from
// growing too big.
type RaftLog struct {
	// for log compaction (snapshot)
	startAt int

	data []LogEntry
}

func newRaftLog() *RaftLog {
	return &RaftLog{
		startAt: 0,
		data: []LogEntry{
			// Raft log is 1-indexed, but we suggest that you view it as 0-indexed,
			// and starting out with an entry (at index=0) that has term 0.
			// That allows the very first AppendEntries RPC to contain 0 as PrevLogIndex,
			// and be a valid index into the log.
			{Term: 0, Command: nil},
		},
	}
}

func (rfLog *RaftLog) getCount() int {
	return rfLog.startAt + len(rfLog.data) - 1
}

func (rfLog *RaftLog) getLastLogIndex() int {
	return rfLog.getCount()
}

func (rfLog *RaftLog) getLogEntry(index int) LogEntry {
	return rfLog.data[index]
}

func (rfLog *RaftLog) getLogEntriesStartedFrom(index int) []LogEntry {
	return rfLog.data[index:]
}

func (rfLog *RaftLog) getXTermAndXIndex(term int, index int) (int, int) {
	xIndex := min(index, rfLog.getLastLogIndex())

	// 1. find the largest term T that is smaller than given term
	for xIndex > 1 && rfLog.getLogEntry(xIndex).Term >= term {
		xIndex -= 1
	}
	xTerm := rfLog.getLogEntry(xIndex).Term

	// 2. find the first log for term T
	for xIndex > 1 && rfLog.getLogEntry(xIndex-1).Term == xTerm {
		xIndex -= 1
	}

	return xTerm, xIndex
}

func (rfLog *RaftLog) replace(startFrom int, entries []LogEntry) {
	rfLog.data = slices.Delete(rfLog.data, startFrom, len(rfLog.data))
	rfLog.data = slices.Concat(rfLog.data, entries)
}

func (rfLog *RaftLog) appendOne(logEntry LogEntry) {
	rfLog.data = append(rfLog.data, logEntry)
}

// remove log entries until index
func (rfLog *RaftLog) trim(index int) {

}
