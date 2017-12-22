use super::node::Node;
use super::transaction::Transaction;
use super::candidate::Candidate;
use std::collections::HashSet;

pub struct MaliciousNode {}

impl MaliciousNode {
    pub fn new(_: f64, _: f64, _: f64, _: i32) -> MaliciousNode {
        MaliciousNode {}
    }
}

impl Node for MaliciousNode {
    fn set_followees(&mut self, _: Vec<bool>) {}

    fn set_pending_txs(&mut self, _: HashSet<Transaction>) {}

    fn send_to_followers(&self) -> HashSet<Transaction> {
        HashSet::new()
    }

    fn reveive_from_followees(&mut self, _: HashSet<Candidate>) {}
}
