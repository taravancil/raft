pub struct Server {
    pub status: ServerStatus,

    // persistent state

    // latest term the server has seen
    pub currentTerm: u64,
    // candidateID that received vote in the current term
    pub votedFor: u32,
    // log entries which contain commands for state machine
    pub log: [LogEntry],

    // Volatile state on all servers

    // index of highest log entry known to be committed
    pub commitIndex: u64,
    // index of highest log entry applied to the state machine
    pub lastApplied: u64,

    // volatile state on leaders only

    // for each server, index of next log entry to send
    // initialized to leader's last log index + 1
    pub nextIndex: [u64],
    // for each server index of higest log entry known to be
    // replicated on the server
    pub matchIndex: [u64],
}

enum ServerStatus {
    Leader,
    Candidate,
    Follower,
}

#[derive(Debug)]
pub enum ServerError {
    // TODO define errors a server may encounter
}
