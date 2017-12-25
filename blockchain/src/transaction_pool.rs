use transaction::Transaction;
use std::collections::HashMap;

pub struct TransactionPool {
    pool: HashMap<[u8; 32], Transaction>,
}

impl TransactionPool {
    fn new() -> Self {
        TransactionPool { pool: HashMap::new() }
    }

    pub fn add_tx(&mut self, tx: Transaction) {
        let hash = tx.hash();
        self.pool.insert(hash, tx);
    }

    pub fn remove_tx(&mut self, hash: &[u8; 32]) {
        self.pool.remove(hash);
    }

    pub fn get_tx(&self, hash: &[u8; 32]) -> &Transaction {
        self.pool.get(hash).unwrap()
    }

    pub fn get_all_txs(&self) -> Vec<&Transaction> {
        let mut res: Vec<&Transaction> = Vec::new();

        for val in self.pool.values() {
            res.push(&val);
        }
        res
    }
}
