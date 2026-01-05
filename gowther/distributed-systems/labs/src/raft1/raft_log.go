package raft

import (
	"slices"
)

type LogEntry struct {
	Term    int
	Command any
}

// Raft log is 1-indexed, and will be trimmed after snapshot to prevent log from
// growing too big.
type RaftLog struct {
	// startAt also means this index is already committed and applied
	// because snapshot happens at the client application, and the client
	// application only can only see the applied log entries
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

// convert data index to log index
func (rfLog *RaftLog) d2l(dataIndex int) int {
	return dataIndex + rfLog.startAt
}

// convert log index to data index
func (rfLog *RaftLog) l2d(logIndex int) int {
	return logIndex - rfLog.startAt
}

// remove log entries until index (not including index)
func (rfLog *RaftLog) trim(index int) {
	dataIndex := rfLog.l2d(index)
	if dataIndex < 1 {
		// has been trimmed already
		return
	}

	rfLog.data = rfLog.data[dataIndex:]
	rfLog.startAt = index
}

// ⚠️ WARNING: The following methods must not depend on or reference `startAt`.

func (rfLog *RaftLog) getLogEntry(index int) (*LogEntry, bool) {
	dataIndex := rfLog.l2d(index)
	if dataIndex < 0 {
		// this log entry has been trimmed
		return nil, false
	}
	return &rfLog.data[dataIndex], true
}

func (rfLog *RaftLog) getCount() int {
	return rfLog.d2l(len(rfLog.data) - 1)
}

func (rfLog *RaftLog) getLastLogIndex() int {
	return rfLog.getCount()
}

func (rfLog *RaftLog) getLastLogTerm() int {
	logEntry, _ := rfLog.getLogEntry(rfLog.getLastLogIndex())
	return logEntry.Term
}

func (rfLog *RaftLog) getLogEntriesStartedFrom(index int) []LogEntry {
	return rfLog.data[rfLog.l2d(index):]
}

func (rfLog *RaftLog) replace(startFrom int, entries []LogEntry) {
	rfLog.data = slices.Delete(rfLog.data, rfLog.l2d(startFrom), len(rfLog.data))
	rfLog.data = slices.Concat(rfLog.data, entries)
}

func (rfLog *RaftLog) appendOne(logEntry LogEntry) {
	rfLog.data = append(rfLog.data, logEntry)
}

func (rfLog *RaftLog) getXTermAndXIndex(term int, index int) (int, int) {
	xIndex := min(index, rfLog.getLastLogIndex())
	var xTerm int

	// 1. find the largest term T that is smaller than given term
	for {
		logEntry, ok := rfLog.getLogEntry(xIndex)
		if !ok {
			return 0, 0
		}

		if logEntry.Term < term {
			xTerm = logEntry.Term
			break
		}

		xIndex -= 1
	}

	// 2. find the first log for term T
	for {
		logEntry, ok := rfLog.getLogEntry(xIndex - 1)
		if !ok {
			return 0, 0
		}

		if logEntry.Term != xTerm {
			break
		}

		xIndex -= 1
	}

	return xTerm, xIndex
}
