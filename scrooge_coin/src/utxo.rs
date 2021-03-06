use std::collections::HashMap;
use super::transaction::TransactionOutput;

#[derive(Eq, PartialEq, Hash)]
pub struct UTXO {
    hash: Vec<u8>,
    index: i32,
}

impl UTXO {
    pub fn new(hash: Vec<u8>, index: i32) -> UTXO {
        UTXO { hash, index }
    }
}

pub struct UTXOPool {
    utxo_map: HashMap<UTXO, TransactionOutput>,
}

impl UTXOPool {
    pub fn add_UTXO(&mut self, utxo: UTXO, tx_out: TransactionOutput) {
        self.utxo_map.insert(utxo, tx_out);
    }

    pub fn remove_UTXO(&mut self, utxo: &UTXO) {
        self.utxo_map.remove(utxo);
    }

    pub fn get_tx_out(&self, utxo: &UTXO) -> &TransactionOutput {
        self.utxo_map.get(utxo).unwrap()
    }

    pub fn get_txs_out(&self) -> Vec<&TransactionOutput> {
        let mut res: Vec<&TransactionOutput> = Vec::new();

        for val in self.utxo_map.values() {
            res.push(&val);
        }
        res
    }

    pub fn contains(&self, utxo: &UTXO) -> bool {
        self.utxo_map.contains_key(utxo)
    }
}
