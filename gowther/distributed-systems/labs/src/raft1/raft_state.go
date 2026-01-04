package raft

type State int

const (
	Leader State = iota
	Follower
	Candidate
)

func (s State) String() string {
	switch s {
	case Leader:
		return "Leader"
	case Follower:
		return "Follower"
	case Candidate:
		return "Candidate"
	default:
		return "Unknown"
	}
}
