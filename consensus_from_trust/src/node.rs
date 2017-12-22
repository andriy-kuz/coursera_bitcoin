use super::transaction::Transaction;
use super::candidate::Candidate;
use std::collections::HashSet;

pub trait Node {
    /// folowers[i] is true if and only if this node follows node i
    /// TODO: accept reference to slice
    fn set_followees(&mut self, folowers: Vec<bool>);
    /// Initialize proposal list of transactions
    fn set_pending_txs(&mut self, txs: HashSet<Transaction>);
    /// Return proposals to send to my followers.
    /// REMEMBER: After final round, behavior of get_proposals changes
    /// and it should return the transactions upon which consensus has been reached
    fn send_to_followers(&self) -> HashSet<Transaction>;
    /// Receive candidates from other nodes
    fn reveive_from_followees(&mut self, candidates: HashSet<Candidate>);
}
