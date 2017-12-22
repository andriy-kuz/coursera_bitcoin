use super::utxo::UTXOPool;
use super::utxo::UTXO;
use super::transaction::Transaction;
use super::crypto::verify_signature;
use std::collections::HashSet;

struct TxHandler {
    utxo_pool: UTXOPool,
}

impl TxHandler {
    pub fn new(utxo_pool: UTXOPool) -> TxHandler {
        {
            TxHandler { utxo_pool }
        }
    }

    fn is_valid(&self, tx: &Transaction) -> bool {
        let mut utxo_set = HashSet::new();
        let mut txs_in_value = 0.0;
        let mut txs_out_value = 0.0;

        for (index, tx_in) in tx.get_inputs().iter().enumerate() {
            //all outputs claimed by tx are in the current UTXO pool
            let utxo = UTXO::new(tx_in.prev_tx_hash.clone(), tx_in.output_index);

            if self.utxo_pool.contains(&utxo) == false {
                return false;
            }
            //the signatures on each input of tx are valid
            let sign_msg = tx.get_raw_data_to_sign(index);
            let tx_out = &self.utxo_pool.get_tx_out(&utxo);
            txs_in_value += tx_out.value;

            if verify_signature(&tx_out.address, &sign_msg, &tx_in.signature) == false {
                return false;
            }
            //no UTXO is claimed multiple times by tx
            if utxo_set.contains(&utxo) {
                return false;
            }
            utxo_set.insert(utxo);
        }
        //all of tx’s output values are non-negative
        for tx_out in tx.get_outputs() {
            if tx_out.value < 0.0 {
                return false;
            }
            txs_out_value += tx_out.value;
        }
        // the sum of tx’s input values is greater than or equal to the sum of its output values
        txs_in_value >= txs_out_value
    }

    pub fn handle_txs(&mut self, txs: Vec<Transaction>) -> Vec<Transaction> {
        let mut res = Vec::new();

        for mut tx in txs {
            if self.is_valid(&tx) == false {
                continue;
            }
            tx.finalize();
            let tx_hash = tx.get_hash();

            for tx_in in tx.get_inputs() {
                let utxo = UTXO::new(tx_in.prev_tx_hash.clone(), tx_in.output_index);
                self.utxo_pool.remove_UTXO(&utxo);
            }

            for (index, tx_out) in tx.get_outputs().iter().enumerate() {
                let utxo = UTXO::new(tx_hash.clone(), index as i32);
                self.utxo_pool.add_UTXO(utxo, (*tx_out).clone());
            }
            res.push(tx);
        }
        res
    }
}
