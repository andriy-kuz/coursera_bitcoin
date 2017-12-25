use crypto;
use std::mem;
use utxo::UTXO;

#[derive(PartialEq, Debug)]
pub struct Transaction {
    hash: [u8; 32],
    input_txs: Vec<TransactionInput>,
    output_txs: Vec<TransactionOutput>,
    coinbase: u8, // bool is not supported in bytevec
}

#[derive(PartialEq, Debug)]
pub struct TransactionInput {
    pub prev_tx_hash: [u8; 32],
    pub output_index: usize,
    pub signature: [u8; 32],
}

#[derive(PartialEq, Debug, Clone)]
pub struct TransactionOutput {
    pub value: f64,
    pub address: Vec<u8>, // RSA public key in PEM format
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            hash: [0; 32],
            input_txs: Vec::new(),
            output_txs: Vec::new(),
            coinbase: 0,
        }
    }

    pub fn new_coinbase(coin: f64, pub_key: Vec<u8>) -> Self {
        let mut tx = Transaction {
            hash: [0; 32],
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

    pub fn add_input_tx(&mut self, prav_tx_hash: [u8; 32], output_index: usize) {
        self.input_txs.push(TransactionInput::new(
            prav_tx_hash,
            output_index,
        ));
    }

    pub fn add_output_tx(&mut self, value: f64, address: Vec<u8>) {
        self.output_txs.push(TransactionOutput::new(value, address));
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

    pub fn raw_data_to_sign(&self, index: usize) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        if index > self.input_txs.len() {
            panic!("Invalid index");
        }
        let input_tx = self.input_txs.get(index).unwrap();
        data.append(&mut input_tx.raw_data_to_sign());
        for output_tx in &self.output_txs {
            data.append(&mut output_tx.raw_data());
        }
        data
    }

    pub fn raw_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for input_tx in &self.input_txs {
            data.append(&mut input_tx.raw_data());
        }

        for output_tx in &self.output_txs {
            data.append(&mut output_tx.raw_data());
        }
        data
    }

    pub fn add_signature(&mut self, signature: [u8; 32], index: usize) {
        if let Some(ref mut intput_tx) = self.input_txs.get_mut(index) {

            intput_tx.add_signature(signature);
        }
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

    pub fn hash(&self) -> [u8; 32] {
        self.hash.clone()
    }

    pub fn finalize(&mut self) {
        self.hash = crypto::double_sha256(&self.raw_data())
    }
}

impl TransactionInput {
    pub fn new(prev_tx_hash: [u8; 32], output_index: usize) -> Self {
        TransactionInput {
            prev_tx_hash,
            output_index,
            signature: [0; 32],
        }
    }

    pub fn add_signature(&mut self, signature: [u8; 32]) {
        self.signature = signature;
    }

    pub fn raw_data_to_sign(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend(self.prev_tx_hash.to_vec().iter().clone());
        unsafe {
            data.extend(
                mem::transmute::<&usize, &[u8; mem::size_of::<usize>()]>(&self.output_index)
                    .iter()
                    .clone(),
            );
        }
        data
    }

    pub fn raw_data(&self) -> Vec<u8> {
        let mut data = self.raw_data_to_sign();
        data.extend(self.signature.to_vec().iter().clone());
        data
    }
}

impl TransactionOutput {
    pub fn new(value: f64, address: Vec<u8>) -> Self {
        TransactionOutput { value, address }
    }

    pub fn raw_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        unsafe {
            data.extend(
                mem::transmute::<&f64, &[u8; mem::size_of::<f64>()]>(&self.value)
                    .iter()
                    .clone(),
            );
        }
        data.append(&mut self.address.clone());
        data
    }
}
