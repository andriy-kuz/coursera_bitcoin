use super::transaction::Transaction;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Candidate {
    _tx: Transaction,
    _sender: usize,
}

impl Candidate {
    pub fn new(tx: Transaction, sender: usize) -> Candidate {
        Candidate {
            _tx: tx,
            _sender: sender,
        }
    }

    pub fn get_sender(&self) -> usize {
        self._sender
    }

    pub fn get_tx(&self) -> Transaction {
        self._tx.clone()
    }
}
