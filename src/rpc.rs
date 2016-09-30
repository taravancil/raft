pub struct Rpc;

pub enum Rpc {
    RequestVote,
    AppendEntry,
}

// AppendEntries is the RPC sent by leaders in order to update the log
pub struct AppendEntries {
    term: u64, // leader's term
    leaderId: u64,
    prevLogIndex: u64, // index of entry immediately preceding new ones
    prevLogTerm: u64,
    entries: [LogEntry],
    leaderCommit: u64, // leader's commitIndex
}

// RequestVote is invoked by candidates to gather votes
pub struct RequestVote {
    term: u64, // candidate's term
    candidateId: u64, // ID of the candidate requesting vote
    lastLogIndex: u64, // index of candidates last log entry
    lastLogTerm: u64, // term of candidate's last log entry
}
