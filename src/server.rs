pub struct Server {
    pub currentTerm: u64, // Latest term the server has seen. 0 on boot
    pub votedFor: u32, // ID of leader the server voted for
    pub log: [LogEntry]
}

enum ServerStatus {
    Leader,
    Candidate,
    Follower,
}
