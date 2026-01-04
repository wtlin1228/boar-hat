package raft

import "slices"

type LogEntry struct {
	Term    int
	Command any
}

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
	// 0 a a a
	// start at 0, count = 3
	// s a a a
	// start at 1, count = 3
	// s s a a
	// start at 2, count = 3
	return rfLog.startAt + len(rfLog.data) - 1
}

func (rfLog *RaftLog) getLastLogIndex() int {
	return rfLog.getCount()
}

func (rfLog *RaftLog) getLogEntry(i int) LogEntry {
	return rfLog.data[i]
}

func (rfLog *RaftLog) getLogEntriesStartedFrom(i int) []LogEntry {
	return rfLog.data[i:]
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

func (rfLog *RaftLog) replaceLogEntries(startFrom int, entries []LogEntry) {
	rfLog.data = slices.Delete(rfLog.data, startFrom, len(rfLog.data))
	rfLog.data = slices.Concat(rfLog.data, entries)
}

func (rfLog *RaftLog) appendOne(logEntry LogEntry) {
	rfLog.data = append(rfLog.data, logEntry)
}
