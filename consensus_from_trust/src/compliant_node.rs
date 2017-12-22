use super::node::Node;
use super::transaction::Transaction;
use super::candidate::Candidate;
use std::collections::HashSet;

pub struct CompliantNode {
    _pending_txs: HashSet<Transaction>,
    _trusted_followee: HashSet<usize>,
    _followees: Vec<bool>,
}
/// CompliantNode refers to a node that follows the rules (not malicious)
impl CompliantNode {
    pub fn new(_: f64, _: f64, _: f64, _: i32) -> CompliantNode {
        CompliantNode {
            _pending_txs: HashSet::new(),
            _trusted_followee: HashSet::new(),
            _followees: Vec::new(),
        }
    }
}

impl Node for CompliantNode {
    fn set_followees(&mut self, folowees: Vec<bool>) {
        self._followees = folowees;
    }

    fn set_pending_txs(&mut self, txs: HashSet<Transaction>) {
        self._pending_txs = txs;
    }

    fn send_to_followers(&self) -> HashSet<Transaction> {
        self._pending_txs.clone()
    }

    fn reveive_from_followees(&mut self, candidates: HashSet<Candidate>) {
        for candidate in &candidates {
            let tx = candidate.get_tx();

            if self._trusted_followee.contains(&candidate.get_sender()) {
                self._pending_txs.insert(tx);
                continue;
            }

            if self._pending_txs.contains(&tx) {
                self._trusted_followee.insert(candidate.get_sender());
                continue;
            }
        }
    }
}
