package raft

import "testing"

func assertCount(t *testing.T, rfLog *RaftLog, expectCount int) {
	count := rfLog.getCount()
	if count != expectCount {
		t.Fatalf("count is %d, but should be %d", count, expectCount)
	}
}

func assertLogEntry(t *testing.T, rfLog *RaftLog, index int, expectTerm int, expectCommand any) {
	logEntry := rfLog.getLogEntry(index)
	if logEntry.Term != expectTerm || logEntry.Command != expectCommand {
		t.Fatalf("#%d log entry is %+v, but should be {Term:%d Command:%v}", index, logEntry, expectTerm, expectCommand)
	}
}

func assertLastLogIndex(t *testing.T, rfLog *RaftLog, expectIndex int) {
	lastLogIndex := rfLog.getLastLogIndex()
	if lastLogIndex != expectIndex {
		t.Fatalf("last log index is %d, but should be %d", lastLogIndex, expectIndex)
	}
}

func TestInitialLog(t *testing.T) {
	rfLog := newRaftLog()

	assertCount(t, rfLog, 0)
	assertLogEntry(t, rfLog, 0, 0, nil)
	assertLastLogIndex(t, rfLog, 0)
}

func TestAppendLog(t *testing.T) {
	rfLog := newRaftLog()
	rfLog.appendOne(LogEntry{Term: 1, Command: 1})

	assertCount(t, rfLog, 1)
	assertLogEntry(t, rfLog, 1, 1, 1)
	assertLastLogIndex(t, rfLog, 1)
}

func TestAppendManyLog(t *testing.T) {
	rfLog := newRaftLog()
	rfLog.appendOne(LogEntry{Term: 1, Command: 1})
	rfLog.appendOne(LogEntry{Term: 1, Command: 2})
	rfLog.appendOne(LogEntry{Term: 1, Command: 3})

	assertCount(t, rfLog, 3)
	assertLogEntry(t, rfLog, 0, 0, nil)
	assertLogEntry(t, rfLog, 1, 1, 1)
	assertLogEntry(t, rfLog, 2, 1, 2)
	assertLogEntry(t, rfLog, 3, 1, 3)
	assertLastLogIndex(t, rfLog, 3)
}

func TestReplaceLog(t *testing.T) {
	rfLog := newRaftLog()
	rfLog.appendOne(LogEntry{Term: 1, Command: 1})
	rfLog.replace(1, []LogEntry{
		{2, 1},
		{2, 2},
		{2, 3},
		{2, 4},
		{2, 5},
		{2, 6},
		{2, 7},
		{2, 8},
		{2, 9},
		{2, 10},
	})

	assertCount(t, rfLog, 10)
	assertLogEntry(t, rfLog, 0, 0, nil)
	assertLogEntry(t, rfLog, 1, 2, 1)
	assertLogEntry(t, rfLog, 2, 2, 2)
	assertLogEntry(t, rfLog, 3, 2, 3)
	assertLogEntry(t, rfLog, 4, 2, 4)
	assertLogEntry(t, rfLog, 5, 2, 5)
	assertLogEntry(t, rfLog, 6, 2, 6)
	assertLogEntry(t, rfLog, 7, 2, 7)
	assertLogEntry(t, rfLog, 8, 2, 8)
	assertLogEntry(t, rfLog, 9, 2, 9)
	assertLogEntry(t, rfLog, 10, 2, 10)
	assertLastLogIndex(t, rfLog, 10)

	rfLog.replace(5, []LogEntry{
		{3, 15},
		{3, 16},
		{3, 17},
	})

	assertCount(t, rfLog, 7)
	assertLogEntry(t, rfLog, 0, 0, nil)
	assertLogEntry(t, rfLog, 1, 2, 1)
	assertLogEntry(t, rfLog, 2, 2, 2)
	assertLogEntry(t, rfLog, 3, 2, 3)
	assertLogEntry(t, rfLog, 4, 2, 4)
	assertLogEntry(t, rfLog, 5, 3, 15)
	assertLogEntry(t, rfLog, 6, 3, 16)
	assertLogEntry(t, rfLog, 7, 3, 17)
	assertLastLogIndex(t, rfLog, 7)
}
