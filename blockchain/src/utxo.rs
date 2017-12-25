use std::collections::HashMap;
use transaction::TransactionOutput;

#[derive(Eq, PartialEq, Hash)]
pub struct UTXO {
    pub hash: [u8; 32],
    pub index: usize,
}

impl UTXO {
    pub fn new(hash: [u8; 32], index: usize) -> Self {
        UTXO { hash, index }
    }
}

pub struct UTXOPool {
    pool: HashMap<UTXO, TransactionOutput>,
}

impl UTXOPool {
    pub fn new() -> Self {
        UTXOPool { pool: HashMap::new() }
    }
    pub fn add_UTXO(&mut self, utxo: UTXO, tx_out: TransactionOutput) {
        self.pool.insert(utxo, tx_out);
    }

    pub fn remove_UTXO(&mut self, utxo: UTXO) {
        self.pool.remove(&utxo);
    }

    pub fn get_tx(&mut self, utxo: &UTXO) -> TransactionOutput {
        self.pool.remove(utxo).unwrap()
    }

    pub fn get_all_txs(self) -> Vec<TransactionOutput> {
        let mut res: Vec<TransactionOutput> = Vec::new();

        for (_, val) in self.pool.into_iter() {
            res.push(val);
        }
        res
    }

    pub fn contains(&self, utxo: &UTXO) -> bool {
        self.pool.contains_key(utxo)
    }
}
