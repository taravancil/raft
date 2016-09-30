pub struct Raft {
    leader: u64, // the ID of the current leader
    servers: [Server], // the servers in the cluster
}
