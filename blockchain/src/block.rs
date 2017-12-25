use crypto;
use transaction::Transaction;

static COINBASE: f64 = 25.0;

pub struct Block {
    hash: [u8; 32],
    prev_hash: [u8; 32],
    coinbase: Transaction,
    txs: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: [u8; 32], address: Vec<u8>) -> Self {
        Block {
            hash: [0; 32],
            prev_hash,
            coinbase: Transaction::new_coinbase(COINBASE, address),
            txs: Vec::new(),
        }
    }

    pub fn coinbase(&self) -> &Transaction {
        &self.coinbase
    }

    pub fn hash(&self) -> &[u8; 32] {
        &self.hash
    }

    pub fn prev_hash(&self) -> &[u8; 32] {
        &self.prev_hash
    }

    pub fn txs(&self) -> &Vec<Transaction> {
        &self.txs
    }

    pub fn tx(&self, index: usize) -> &Transaction {
        self.txs.get(index).unwrap()
    }

    pub fn add_tx(&mut self, tx: Transaction) {
        self.txs.push(tx);
    }

    pub fn raw_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend(self.prev_hash.to_vec().iter().clone());
        for tx in &self.txs {
            data.extend(&tx.raw_data());
        }
        data
    }

    pub fn finalize(&mut self) {
        self.hash = crypto::double_sha256(&self.raw_data());
    }
}