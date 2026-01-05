package raft

import (
	"testing"
)

func assertCount(t *testing.T, rfLog *RaftLog, expectCount int) {
	count := rfLog.getCount()
	if count != expectCount {
		t.Fatalf("count is %d, but should be %d", count, expectCount)
	}
}

func assertLogEntry(t *testing.T, rfLog *RaftLog, index int, expectTerm int, expectCommand any) {
	logEntry, _ := rfLog.getLogEntry(index)
	if logEntry.Term != expectTerm || logEntry.Command != expectCommand {
		t.Fatalf("#%d log entry is %+v, but should be {Term:%d Command:%v}", index, logEntry, expectTerm, expectCommand)
	}
}

func assertTrimmedLogEntry(t *testing.T, rfLog *RaftLog, index int) {
	logEntry, ok := rfLog.getLogEntry(index)
	if logEntry != nil || ok != false {
		t.Fatalf("#%d log entry is %+v, but should be nil", index, logEntry)
	}
}

func assertLastLogIndex(t *testing.T, rfLog *RaftLog, expectIndex int) {
	lastLogIndex := rfLog.getLastLogIndex()
	if lastLogIndex != expectIndex {
		t.Fatalf("last log index is %d, but should be %d", lastLogIndex, expectIndex)
	}
}

func assertLastLogTerm(t *testing.T, rfLog *RaftLog, expectTerm int) {
	lastLogTerm := rfLog.getLastLogTerm()
	if lastLogTerm != expectTerm {
		t.Fatalf("last log term is %d, but should be %d", lastLogTerm, expectTerm)
	}
}

func TestInitialLog(t *testing.T) {
	rfLog := newRaftLog()

	assertCount(t, rfLog, 0)
	assertLogEntry(t, rfLog, 0, 0, nil)
	assertLastLogIndex(t, rfLog, 0)
	assertLastLogTerm(t, rfLog, 0)
}

func TestAppendLog(t *testing.T) {
	rfLog := newRaftLog()
	rfLog.appendOne(LogEntry{Term: 1, Command: 1})

	assertCount(t, rfLog, 1)
	assertLogEntry(t, rfLog, 1, 1, 1)
	assertLastLogIndex(t, rfLog, 1)
	assertLastLogTerm(t, rfLog, 1)
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
	assertLastLogTerm(t, rfLog, 1)
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
	assertLastLogTerm(t, rfLog, 2)

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
	assertLastLogTerm(t, rfLog, 3)
}

func TestTrimLog(t *testing.T) {
	rfLog := newRaftLog()
	trimmedRfLog := newRaftLog()

	for i := 1; i <= 10; i++ {
		rfLog.appendOne(LogEntry{Term: 1, Command: i})
		trimmedRfLog.appendOne(LogEntry{Term: 1, Command: i})
	}

	trimmedRfLog.trim(3)

	assertCount(t, trimmedRfLog, rfLog.getCount())
	for i := 3; i <= 10; i++ {
		expectLogEntry, _ := rfLog.getLogEntry(i)
		assertLogEntry(t, trimmedRfLog, i, expectLogEntry.Term, expectLogEntry.Command)
	}
	assertLastLogIndex(t, trimmedRfLog, rfLog.getLastLogIndex())
	assertLastLogTerm(t, trimmedRfLog, rfLog.getLastLogTerm())
}

func TestTrimTheSameRangeLog(t *testing.T) {
	trimmedRfLog := newRaftLog()

	for i := 1; i <= 10; i++ {
		trimmedRfLog.appendOne(LogEntry{Term: 1, Command: i})
	}

	trimmedRfLog.trim(3)
	startAt1 := trimmedRfLog.startAt
	dataLen1 := len(trimmedRfLog.data)

	trimmedRfLog.trim(3)
	startAt2 := trimmedRfLog.startAt
	dataLen2 := len(trimmedRfLog.data)

	if startAt1 != startAt2 || dataLen1 != dataLen2 {
		t.Fatal("trim the same range twice shouldn't change anything")
	}
}

func TestAppendOnTrimmedLog(t *testing.T) {
	rfLog := newRaftLog()
	trimmedRfLog := newRaftLog()

	for i := 1; i <= 10; i++ {
		rfLog.appendOne(LogEntry{Term: 1, Command: i})
		trimmedRfLog.appendOne(LogEntry{Term: 1, Command: i})
	}

	trimmedRfLog.trim(3)

	rfLog.appendOne(LogEntry{Term: 1, Command: 11})
	rfLog.appendOne(LogEntry{Term: 1, Command: 12})
	rfLog.appendOne(LogEntry{Term: 1, Command: 13})

	trimmedRfLog.appendOne(LogEntry{Term: 1, Command: 11})
	trimmedRfLog.appendOne(LogEntry{Term: 1, Command: 12})
	trimmedRfLog.appendOne(LogEntry{Term: 1, Command: 13})

	assertCount(t, trimmedRfLog, rfLog.getCount())
	for i := 3; i <= 13; i++ {
		expectLogEntry, _ := rfLog.getLogEntry(i)
		assertLogEntry(t, trimmedRfLog, i, expectLogEntry.Term, expectLogEntry.Command)
	}
	assertLastLogIndex(t, trimmedRfLog, rfLog.getLastLogIndex())
	assertLastLogTerm(t, trimmedRfLog, rfLog.getLastLogTerm())
}

func TestReplaceOnTrimmedLog(t *testing.T) {
	rfLog := newRaftLog()
	trimmedRfLog := newRaftLog()

	for i := 1; i <= 10; i++ {
		rfLog.appendOne(LogEntry{Term: 1, Command: i})
		trimmedRfLog.appendOne(LogEntry{Term: 1, Command: i})
	}

	trimmedRfLog.trim(3)

	rfLog.replace(5, []LogEntry{
		{2, 15},
		{2, 16},
		{2, 17},
	})
	trimmedRfLog.replace(5, []LogEntry{
		{2, 15},
		{2, 16},
		{2, 17},
	})

	assertCount(t, trimmedRfLog, rfLog.getCount())
	for i := 3; i <= 7; i++ {
		expectLogEntry, _ := rfLog.getLogEntry(i)
		assertLogEntry(t, trimmedRfLog, i, expectLogEntry.Term, expectLogEntry.Command)
	}
	assertLastLogIndex(t, trimmedRfLog, rfLog.getLastLogIndex())
	assertLastLogTerm(t, trimmedRfLog, rfLog.getLastLogTerm())
}

func TestGetLogEntryOnTrimmedLog(t *testing.T) {
	trimmedRfLog := newRaftLog()

	for i := 1; i <= 10; i++ {
		trimmedRfLog.appendOne(LogEntry{Term: 1, Command: i})
	}

	trimmedRfLog.trim(3)

	for i := 0; i < 3; i++ {
		assertTrimmedLogEntry(t, trimmedRfLog, i)
	}

	for i := 3; i <= 10; i++ {
		assertLogEntry(t, trimmedRfLog, i, 1, i)
	}
}
