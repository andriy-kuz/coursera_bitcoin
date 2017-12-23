use bytevec::{ByteEncodable};
use utxo::UTXO;
use crypto;

bytevec_decl! {
#[derive(PartialEq, Debug)]
pub struct Transaction {
    hash: Vec<u8>,
    input_txs: Vec<TransactionInput>,
    output_txs: Vec<TransactionOutput>,
    coinbase: u8 // bool is not supported in bytevec
}

#[derive(PartialEq, Debug, Clone)]
pub struct TransactionOutput {
    value: f64,
    address: Vec<u8> // RSA public key in PEM format
}

#[derive(PartialEq, Debug)]
pub struct TransactionInput {
    prev_tx_hash: Vec<u8>,
    output_index: usize,
    signature: Vec<u8>
}
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            hash: Vec::new(),
            input_txs: Vec::new(),
            output_txs: Vec::new(),
            coinbase: 0,
        }
    }

    pub fn new_value(coin: f64, pub_key: Vec<u8>) -> Transaction {
        let mut tx = Transaction {
            hash: Vec::new(),
            input_txs: Vec::new(),
            output_txs: Vec::new(),
            coinbase: 1,
        };
        tx.add_output_tx(coin, pub_key);
        tx.finalize();
        tx
    }

    pub fn is_coinbase(&self) -> u8 {
        return self.coinbase;
    }

    pub fn add_input_tx(&mut self, prav_tx_hash: Vec<u8>, output_index: usize) {
        let tx = TransactionInput::new(prav_tx_hash, output_index);
        self.input_txs.push(tx);
    }

    pub fn add_output_tx(&mut self, value: f64, address: Vec<u8>) {
        let tx = TransactionOutput::new(value, address);
        self.output_txs.push(tx);
    }

    pub fn remove_input_tx(&mut self, index: usize) {
        self.input_txs.remove(index);
    }

    pub fn remove_input_utxo(&mut self, ut: UTXO) {
        self.input_txs.retain(|ref tx| {
            let tx_ut = UTXO::new(tx.prev_tx_hash.clone(), tx.output_index);
            tx_ut != ut
        });
    }

    pub fn get_raw_data_to_sign(&self, index: usize) -> Vec<u8> {
        let mut sig_data: Vec<u8> = Vec::new();

        if index > self.input_txs.len() {
            panic!("Invalid index");
        }
        let input_tx = self.input_txs.get(index).unwrap();
        let mut data = input_tx.encode::<u32>().unwrap();
        sig_data.append(&mut data);
        let mut data = self.output_txs.encode::<u32>().unwrap();
        sig_data.append(&mut data);
        sig_data
    }

    pub fn add_signature(&mut self, signature: Vec<u8>, index: usize) {
        if let Some(ref mut intput_tx) = self.input_txs.get_mut(index) {
            intput_tx.add_signature(signature);
        }
    }

    pub fn finalize(&mut self) {
        let raw_tx = self.get_raw_data();
        self.hash = crypto::double_sha256(&raw_tx)
    }

    fn get_raw_data(&self) -> Vec<u8> {
        self.encode::<u32>().unwrap()
    }

    pub fn set_hash(&mut self, hash: Vec<u8>) {
        self.hash = hash;
    }

    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    pub fn get_inputs(&self) -> &Vec<TransactionInput> {
        &self.input_txs
    }

    pub fn get_outputs(&self) -> &Vec<TransactionOutput> {
        &self.output_txs
    }

    pub fn get_input(&self, index: usize) -> &TransactionInput {
        self.input_txs.get(index).unwrap()
    }

    pub fn get_output(&self, index: usize) -> &TransactionOutput {
        self.output_txs.get(index).unwrap()
    }

    pub fn inputs_len(&self) -> usize {
        self.input_txs.len()
    }

    pub fn outputs_len(&self) -> usize {
        self.output_txs.len()
    }

    pub fn hash(&self) -> Vec<u8> {
        let data = self.get_raw_data();
        crypto::double_sha256(&data)
    }
}

impl TransactionInput {
    pub fn new(prev_tx_hash: Vec<u8>, output_index: usize) -> TransactionInput {
        TransactionInput {
            prev_tx_hash,
            output_index,
            signature: Vec::new(),
        }
    }

    pub fn add_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature;
    }

    pub fn hash(&self) -> Vec<u8> {
        let data = self.encode::<u32>().unwrap();
        crypto::double_sha256(&data)
    }
}

impl TransactionOutput {
    pub fn new(value: f64, address: Vec<u8>) -> TransactionOutput {
        TransactionOutput { value, address }
    }

    pub fn hash(&self) -> Vec<u8> {
        let data = self.encode::<u32>().unwrap();
        crypto::double_sha256(&data)
    }
}
