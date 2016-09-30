pub struct Raft {
    leader: u64, // the ID of the current leader
    servers: [Server], // the servers in the cluster
}

impl Raft {
    /// Create a new `Raft` client
    pub fn new() -> Raft {
    }

    /// Append an entry to the cluster's state machine
    pub fn append() -> Result<()> {
    }
}
