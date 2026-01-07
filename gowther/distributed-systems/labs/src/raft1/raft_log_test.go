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

func assertFirstLogIndex(t *testing.T, rfLog *RaftLog, expectIndex int) {
	firstLogIndex := rfLog.getFirstLogIndex()
	if firstLogIndex != expectIndex {
		t.Fatalf("first log index is %d, but should be %d", firstLogIndex, expectIndex)
	}
}

func assertLastLogTerm(t *testing.T, rfLog *RaftLog, expectTerm int) {
	lastLogTerm := rfLog.getLastLogTerm()
	if lastLogTerm != expectTerm {
		t.Fatalf("last log term is %d, but should be %d", lastLogTerm, expectTerm)
	}
}

func assertFirstLogTerm(t *testing.T, rfLog *RaftLog, expectTerm int) {
	firstLogTerm := rfLog.getFirstLogTerm()
	if firstLogTerm != expectTerm {
		t.Fatalf("first log term is %d, but should be %d", firstLogTerm, expectTerm)
	}
}

func assertXIndex(t *testing.T, rfLog *RaftLog, term int, index int, expectXIndex int) {
	xIndex := rfLog.getXIndex(term, index)
	if xIndex != expectXIndex {
		t.Fatalf("xIndex is %d, but should be %d", xIndex, expectXIndex)
	}
}

func TestInitialLog(t *testing.T) {
	rfLog := newRaftLog()

	assertCount(t, rfLog, 0)
	assertLogEntry(t, rfLog, 0, 0, nil)
	assertFirstLogIndex(t, rfLog, 0)
	assertFirstLogTerm(t, rfLog, 0)
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

func TestFirstEntryOnTrimmedLog(t *testing.T) {
	trimmedRfLog := newRaftLog()

	for i := 1; i <= 10; i++ {
		trimmedRfLog.appendOne(LogEntry{Term: 1, Command: i})
	}

	assertFirstLogIndex(t, trimmedRfLog, 0)
	assertFirstLogTerm(t, trimmedRfLog, 0)
	trimmedRfLog.trim(3)
	assertFirstLogIndex(t, trimmedRfLog, 3)
	assertFirstLogTerm(t, trimmedRfLog, 1)
}

func TestGetXTermAndXIndexLog(t *testing.T) {
	rfLog := newRaftLog()

	// index: 0 1 2 3 4 5 6 7 8 9 10
	// term:  0 2 2 2 4 4 4 5 5 5 8
	rfLog.appendOne(LogEntry{Term: 2, Command: 21})
	rfLog.appendOne(LogEntry{Term: 2, Command: 22})
	rfLog.appendOne(LogEntry{Term: 2, Command: 23})
	rfLog.appendOne(LogEntry{Term: 4, Command: 44})
	rfLog.appendOne(LogEntry{Term: 4, Command: 45})
	rfLog.appendOne(LogEntry{Term: 4, Command: 46})
	rfLog.appendOne(LogEntry{Term: 5, Command: 57})
	rfLog.appendOne(LogEntry{Term: 5, Command: 58})
	rfLog.appendOne(LogEntry{Term: 5, Command: 59})
	rfLog.appendOne(LogEntry{Term: 8, Command: 80})

	assertXIndex(t, rfLog, 99, 11, 10)
	assertXIndex(t, rfLog, 99, 10, 10)
	assertXIndex(t, rfLog, 99, 9, 7)
	assertXIndex(t, rfLog, 99, 8, 7)
	assertXIndex(t, rfLog, 99, 7, 7)
	assertXIndex(t, rfLog, 99, 6, 4)
	assertXIndex(t, rfLog, 99, 5, 4)
	assertXIndex(t, rfLog, 99, 4, 4)
	assertXIndex(t, rfLog, 99, 3, 1)
	assertXIndex(t, rfLog, 99, 2, 1)
	assertXIndex(t, rfLog, 99, 1, 1)
	assertXIndex(t, rfLog, 99, 0, 1)

	assertXIndex(t, rfLog, 9, 99, 10)
	assertXIndex(t, rfLog, 8, 99, 7)
	assertXIndex(t, rfLog, 7, 99, 7)
	assertXIndex(t, rfLog, 6, 99, 7)
	assertXIndex(t, rfLog, 5, 99, 4)
	assertXIndex(t, rfLog, 4, 99, 1)
	assertXIndex(t, rfLog, 3, 99, 1)
	assertXIndex(t, rfLog, 2, 99, 1)
	assertXIndex(t, rfLog, 1, 99, 1)
	assertXIndex(t, rfLog, 0, 99, 1)

	assertXIndex(t, rfLog, 8, 10, 7)
	assertXIndex(t, rfLog, 5, 9, 4)
	assertXIndex(t, rfLog, 5, 8, 4)
	assertXIndex(t, rfLog, 5, 7, 4)
	assertXIndex(t, rfLog, 4, 6, 1)
	assertXIndex(t, rfLog, 4, 5, 1)
	assertXIndex(t, rfLog, 4, 4, 1)
	assertXIndex(t, rfLog, 2, 3, 1)
	assertXIndex(t, rfLog, 2, 2, 1)
	assertXIndex(t, rfLog, 2, 1, 1)
	assertXIndex(t, rfLog, 0, 0, 1)

	assertXIndex(t, rfLog, 7, 11, 7)
	assertXIndex(t, rfLog, 7, 10, 7)
	assertXIndex(t, rfLog, 7, 9, 7)
	assertXIndex(t, rfLog, 7, 8, 7)
	assertXIndex(t, rfLog, 7, 7, 7)
	assertXIndex(t, rfLog, 7, 6, 4)
	assertXIndex(t, rfLog, 7, 5, 4)
	assertXIndex(t, rfLog, 7, 4, 4)
	assertXIndex(t, rfLog, 7, 3, 1)
	assertXIndex(t, rfLog, 7, 2, 1)
	assertXIndex(t, rfLog, 7, 1, 1)
	assertXIndex(t, rfLog, 7, 0, 1)
}

func TestGetXTermAndXIndexOnTrimmedLog(t *testing.T) {
	trimmedRfLog := newRaftLog()

	// index: 0 1 2 3 4 5 6 7 8 9 10
	// term:  0 2 2 2 4 4 4 5 5 5 8
	trimmedRfLog.appendOne(LogEntry{Term: 2, Command: 21})
	trimmedRfLog.appendOne(LogEntry{Term: 2, Command: 22})
	trimmedRfLog.appendOne(LogEntry{Term: 2, Command: 23})
	trimmedRfLog.appendOne(LogEntry{Term: 4, Command: 44})
	trimmedRfLog.appendOne(LogEntry{Term: 4, Command: 45})
	trimmedRfLog.appendOne(LogEntry{Term: 4, Command: 46})
	trimmedRfLog.appendOne(LogEntry{Term: 5, Command: 57})
	trimmedRfLog.appendOne(LogEntry{Term: 5, Command: 58})
	trimmedRfLog.appendOne(LogEntry{Term: 5, Command: 59})
	trimmedRfLog.appendOne(LogEntry{Term: 8, Command: 80})

	trimmedRfLog.trim(1)

	assertXIndex(t, trimmedRfLog, 99, 11, 10)
	assertXIndex(t, trimmedRfLog, 99, 10, 10)
	assertXIndex(t, trimmedRfLog, 99, 9, 7)
	assertXIndex(t, trimmedRfLog, 99, 8, 7)
	assertXIndex(t, trimmedRfLog, 99, 7, 7)
	assertXIndex(t, trimmedRfLog, 99, 6, 4)
	assertXIndex(t, trimmedRfLog, 99, 5, 4)
	assertXIndex(t, trimmedRfLog, 99, 4, 4)
	assertXIndex(t, trimmedRfLog, 99, 3, 2)
	assertXIndex(t, trimmedRfLog, 99, 2, 2)
	assertXIndex(t, trimmedRfLog, 99, 1, 2)
	assertXIndex(t, trimmedRfLog, 99, 0, 2)

	assertXIndex(t, trimmedRfLog, 9, 99, 10)
	assertXIndex(t, trimmedRfLog, 8, 99, 7)
	assertXIndex(t, trimmedRfLog, 7, 99, 7)
	assertXIndex(t, trimmedRfLog, 6, 99, 7)
	assertXIndex(t, trimmedRfLog, 5, 99, 4)
	assertXIndex(t, trimmedRfLog, 4, 99, 2)
	assertXIndex(t, trimmedRfLog, 3, 99, 2)
	assertXIndex(t, trimmedRfLog, 2, 99, 2)
	assertXIndex(t, trimmedRfLog, 1, 99, 2)
	assertXIndex(t, trimmedRfLog, 0, 99, 2)

	assertXIndex(t, trimmedRfLog, 8, 10, 7)
	assertXIndex(t, trimmedRfLog, 5, 9, 4)
	assertXIndex(t, trimmedRfLog, 5, 8, 4)
	assertXIndex(t, trimmedRfLog, 5, 7, 4)
	assertXIndex(t, trimmedRfLog, 4, 6, 2)
	assertXIndex(t, trimmedRfLog, 4, 5, 2)
	assertXIndex(t, trimmedRfLog, 4, 4, 2)
	assertXIndex(t, trimmedRfLog, 2, 3, 2)
	assertXIndex(t, trimmedRfLog, 2, 2, 2)
	assertXIndex(t, trimmedRfLog, 2, 1, 2)
	assertXIndex(t, trimmedRfLog, 0, 0, 2)

	assertXIndex(t, trimmedRfLog, 7, 11, 7)
	assertXIndex(t, trimmedRfLog, 7, 10, 7)
	assertXIndex(t, trimmedRfLog, 7, 9, 7)
	assertXIndex(t, trimmedRfLog, 7, 8, 7)
	assertXIndex(t, trimmedRfLog, 7, 7, 7)
	assertXIndex(t, trimmedRfLog, 7, 6, 4)
	assertXIndex(t, trimmedRfLog, 7, 5, 4)
	assertXIndex(t, trimmedRfLog, 7, 4, 4)
	assertXIndex(t, trimmedRfLog, 7, 3, 2)
	assertXIndex(t, trimmedRfLog, 7, 2, 2)
	assertXIndex(t, trimmedRfLog, 7, 1, 2)
	assertXIndex(t, trimmedRfLog, 7, 0, 2)

	trimmedRfLog.trim(2)

	assertXIndex(t, trimmedRfLog, 99, 11, 10)
	assertXIndex(t, trimmedRfLog, 99, 10, 10)
	assertXIndex(t, trimmedRfLog, 99, 9, 7)
	assertXIndex(t, trimmedRfLog, 99, 8, 7)
	assertXIndex(t, trimmedRfLog, 99, 7, 7)
	assertXIndex(t, trimmedRfLog, 99, 6, 4)
	assertXIndex(t, trimmedRfLog, 99, 5, 4)
	assertXIndex(t, trimmedRfLog, 99, 4, 4)
	assertXIndex(t, trimmedRfLog, 99, 3, 3)
	assertXIndex(t, trimmedRfLog, 99, 2, 3)
	assertXIndex(t, trimmedRfLog, 99, 1, 3)
	assertXIndex(t, trimmedRfLog, 99, 0, 3)

	assertXIndex(t, trimmedRfLog, 9, 99, 10)
	assertXIndex(t, trimmedRfLog, 8, 99, 7)
	assertXIndex(t, trimmedRfLog, 7, 99, 7)
	assertXIndex(t, trimmedRfLog, 6, 99, 7)
	assertXIndex(t, trimmedRfLog, 5, 99, 4)
	assertXIndex(t, trimmedRfLog, 4, 99, 3)
	assertXIndex(t, trimmedRfLog, 3, 99, 3)
	assertXIndex(t, trimmedRfLog, 2, 99, 3)
	assertXIndex(t, trimmedRfLog, 1, 99, 3)
	assertXIndex(t, trimmedRfLog, 0, 99, 3)

	assertXIndex(t, trimmedRfLog, 8, 10, 7)
	assertXIndex(t, trimmedRfLog, 5, 9, 4)
	assertXIndex(t, trimmedRfLog, 5, 8, 4)
	assertXIndex(t, trimmedRfLog, 5, 7, 4)
	assertXIndex(t, trimmedRfLog, 4, 6, 3)
	assertXIndex(t, trimmedRfLog, 4, 5, 3)
	assertXIndex(t, trimmedRfLog, 4, 4, 3)
	assertXIndex(t, trimmedRfLog, 2, 3, 3)
	assertXIndex(t, trimmedRfLog, 2, 2, 3)
	assertXIndex(t, trimmedRfLog, 2, 1, 3)
	assertXIndex(t, trimmedRfLog, 0, 0, 3)

	assertXIndex(t, trimmedRfLog, 7, 11, 7)
	assertXIndex(t, trimmedRfLog, 7, 10, 7)
	assertXIndex(t, trimmedRfLog, 7, 9, 7)
	assertXIndex(t, trimmedRfLog, 7, 8, 7)
	assertXIndex(t, trimmedRfLog, 7, 7, 7)
	assertXIndex(t, trimmedRfLog, 7, 6, 4)
	assertXIndex(t, trimmedRfLog, 7, 5, 4)
	assertXIndex(t, trimmedRfLog, 7, 4, 4)
	assertXIndex(t, trimmedRfLog, 7, 3, 3)
	assertXIndex(t, trimmedRfLog, 7, 2, 3)
	assertXIndex(t, trimmedRfLog, 7, 1, 3)
	assertXIndex(t, trimmedRfLog, 7, 0, 3)

	trimmedRfLog.trim(3)

	assertXIndex(t, trimmedRfLog, 99, 11, 10)
	assertXIndex(t, trimmedRfLog, 99, 10, 10)
	assertXIndex(t, trimmedRfLog, 99, 9, 7)
	assertXIndex(t, trimmedRfLog, 99, 8, 7)
	assertXIndex(t, trimmedRfLog, 99, 7, 7)
	assertXIndex(t, trimmedRfLog, 99, 6, 4)
	assertXIndex(t, trimmedRfLog, 99, 5, 4)
	assertXIndex(t, trimmedRfLog, 99, 4, 4)
	assertXIndex(t, trimmedRfLog, 99, 3, 4)
	assertXIndex(t, trimmedRfLog, 99, 2, 4)
	assertXIndex(t, trimmedRfLog, 99, 1, 4)
	assertXIndex(t, trimmedRfLog, 99, 0, 4)

	assertXIndex(t, trimmedRfLog, 9, 99, 10)
	assertXIndex(t, trimmedRfLog, 8, 99, 7)
	assertXIndex(t, trimmedRfLog, 7, 99, 7)
	assertXIndex(t, trimmedRfLog, 6, 99, 7)
	assertXIndex(t, trimmedRfLog, 5, 99, 4)
	assertXIndex(t, trimmedRfLog, 4, 99, 4)
	assertXIndex(t, trimmedRfLog, 3, 99, 4)
	assertXIndex(t, trimmedRfLog, 2, 99, 4)
	assertXIndex(t, trimmedRfLog, 1, 99, 4)
	assertXIndex(t, trimmedRfLog, 0, 99, 4)

	assertXIndex(t, trimmedRfLog, 8, 10, 7)
	assertXIndex(t, trimmedRfLog, 5, 9, 4)
	assertXIndex(t, trimmedRfLog, 5, 8, 4)
	assertXIndex(t, trimmedRfLog, 5, 7, 4)
	assertXIndex(t, trimmedRfLog, 4, 6, 4)
	assertXIndex(t, trimmedRfLog, 4, 5, 4)
	assertXIndex(t, trimmedRfLog, 4, 4, 4)
	assertXIndex(t, trimmedRfLog, 2, 3, 4)
	assertXIndex(t, trimmedRfLog, 2, 2, 4)
	assertXIndex(t, trimmedRfLog, 2, 1, 4)
	assertXIndex(t, trimmedRfLog, 0, 0, 4)

	assertXIndex(t, trimmedRfLog, 7, 11, 7)
	assertXIndex(t, trimmedRfLog, 7, 10, 7)
	assertXIndex(t, trimmedRfLog, 7, 9, 7)
	assertXIndex(t, trimmedRfLog, 7, 8, 7)
	assertXIndex(t, trimmedRfLog, 7, 7, 7)
	assertXIndex(t, trimmedRfLog, 7, 6, 4)
	assertXIndex(t, trimmedRfLog, 7, 5, 4)
	assertXIndex(t, trimmedRfLog, 7, 4, 4)
	assertXIndex(t, trimmedRfLog, 7, 3, 4)
	assertXIndex(t, trimmedRfLog, 7, 2, 4)
	assertXIndex(t, trimmedRfLog, 7, 1, 4)
	assertXIndex(t, trimmedRfLog, 7, 0, 4)

	trimmedRfLog.trim(10)
	assertXIndex(t, trimmedRfLog, 99, 11, 11)
	assertXIndex(t, trimmedRfLog, 99, 10, 11)
	assertXIndex(t, trimmedRfLog, 99, 9, 11)
	assertXIndex(t, trimmedRfLog, 99, 8, 11)
	assertXIndex(t, trimmedRfLog, 99, 7, 11)
	assertXIndex(t, trimmedRfLog, 99, 6, 11)
	assertXIndex(t, trimmedRfLog, 99, 5, 11)
	assertXIndex(t, trimmedRfLog, 99, 4, 11)
	assertXIndex(t, trimmedRfLog, 99, 3, 11)
	assertXIndex(t, trimmedRfLog, 99, 2, 11)
	assertXIndex(t, trimmedRfLog, 99, 1, 11)
	assertXIndex(t, trimmedRfLog, 99, 0, 11)

	assertXIndex(t, trimmedRfLog, 9, 99, 11)
	assertXIndex(t, trimmedRfLog, 8, 99, 11)
	assertXIndex(t, trimmedRfLog, 7, 99, 11)
	assertXIndex(t, trimmedRfLog, 6, 99, 11)
	assertXIndex(t, trimmedRfLog, 5, 99, 11)
	assertXIndex(t, trimmedRfLog, 4, 99, 11)
	assertXIndex(t, trimmedRfLog, 3, 99, 11)
	assertXIndex(t, trimmedRfLog, 2, 99, 11)
	assertXIndex(t, trimmedRfLog, 1, 99, 11)
	assertXIndex(t, trimmedRfLog, 0, 99, 11)

	assertXIndex(t, trimmedRfLog, 8, 10, 11)
	assertXIndex(t, trimmedRfLog, 5, 9, 11)
	assertXIndex(t, trimmedRfLog, 5, 8, 11)
	assertXIndex(t, trimmedRfLog, 5, 7, 11)
	assertXIndex(t, trimmedRfLog, 4, 6, 11)
	assertXIndex(t, trimmedRfLog, 4, 5, 11)
	assertXIndex(t, trimmedRfLog, 4, 4, 11)
	assertXIndex(t, trimmedRfLog, 2, 3, 11)
	assertXIndex(t, trimmedRfLog, 2, 2, 11)
	assertXIndex(t, trimmedRfLog, 2, 1, 11)
	assertXIndex(t, trimmedRfLog, 0, 0, 11)

	assertXIndex(t, trimmedRfLog, 7, 11, 11)
	assertXIndex(t, trimmedRfLog, 7, 10, 11)
	assertXIndex(t, trimmedRfLog, 7, 9, 11)
	assertXIndex(t, trimmedRfLog, 7, 8, 11)
	assertXIndex(t, trimmedRfLog, 7, 7, 11)
	assertXIndex(t, trimmedRfLog, 7, 6, 11)
	assertXIndex(t, trimmedRfLog, 7, 5, 11)
	assertXIndex(t, trimmedRfLog, 7, 4, 11)
	assertXIndex(t, trimmedRfLog, 7, 3, 11)
	assertXIndex(t, trimmedRfLog, 7, 2, 11)
	assertXIndex(t, trimmedRfLog, 7, 1, 11)
	assertXIndex(t, trimmedRfLog, 7, 0, 11)
}
