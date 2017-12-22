extern crate hex;
extern crate openssl;

use std::mem;
use self::openssl::sha;
use super::utxo::UTXO;

pub struct Transaction {
    hash: Vec<u8>,
    input_txs: Vec<TransactionInput>,
    output_txs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            hash: Vec::new(),
            input_txs: Vec::new(),
            output_txs: Vec::new(),
        }
    }

    pub fn add_input_tx(&mut self, input_tx: TransactionInput) {
        self.input_txs.push(input_tx);
    }

    pub fn add_output_tx(&mut self, output_tx: TransactionOutput) {
        self.output_txs.push(output_tx);
    }

    pub fn remove_input_tx(&mut self, index: i32) {
        self.input_txs.remove(index as usize);
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

        let input_tx = self.input_txs.get(index as usize).unwrap();
        // add appropriate current input transaction
        sig_data.extend(input_tx.prev_tx_hash.iter().clone());
        unsafe {
            sig_data.extend(
                mem::transmute::<&i32, &[u8; 4]>(&input_tx.output_index)
                    .iter()
                    .clone(),
            );
        }
        // add output transactions
        for output_tx in &self.output_txs {
            unsafe {
                sig_data.extend(
                    mem::transmute::<&f64, &[u8; 8]>(&output_tx.value)
                        .iter()
                        .clone(),
                );
            }
            sig_data.append(&mut output_tx.address.clone());
        }
        sig_data
    }

    pub fn add_signature(&mut self, signature: Vec<u8>, index: i32) {
        if let Some(ref mut intput_tx) = self.input_txs.get_mut(index as usize) {
            intput_tx.add_signature(signature);
        }
    }

    pub fn finalize(&mut self) {
        let mut hasher = sha::Sha256::new();
        let raw_tx = self.get_raw_tx();
        hasher.update(&raw_tx);
        self.hash = hasher.finish().to_vec();
    }

    fn get_raw_tx(&self) -> Vec<u8> {
        let mut tx_data: Vec<u8> = Vec::new();

        for input_tx in &self.input_txs {
            // add appropriate current input transaction
            tx_data.extend(input_tx.prev_tx_hash.iter().clone());
            unsafe {
                tx_data.extend(
                    mem::transmute::<&i32, &[u8; 4]>(&input_tx.output_index)
                        .iter()
                        .clone(),
                );
            }
            tx_data.extend(input_tx.prev_tx_hash.iter().clone());
        }
        // add output transactions
        for output_tx in &self.output_txs {
            unsafe {
                tx_data.extend(
                    mem::transmute::<&f64, &[u8; 8]>(&output_tx.value)
                        .iter()
                        .clone(),
                );
            }
            tx_data.append(&mut output_tx.address.clone());
        }
        tx_data
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

    fn get_input(&self, index: i32) -> &TransactionInput {
        self.input_txs.get(index as usize).unwrap()
    }

    fn get_output(&self, index: i32) -> &TransactionOutput {
        self.output_txs.get(index as usize).unwrap()
    }

    fn inputs_size(&self) -> usize {
        self.input_txs.len()
    }

    fn outputs_size(&self) -> usize {
        self.output_txs.len()
    }
}

pub struct TransactionInput {
    pub prev_tx_hash: Vec<u8>,
    pub output_index: i32,
    pub signature: Vec<u8>,
}

impl TransactionInput {
    pub fn new(prev_tx_hash: Vec<u8>, output_index: i32) -> TransactionInput {
        TransactionInput {
            prev_tx_hash,
            output_index,
            signature: Vec::new(),
        }
    }

    pub fn add_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature;
    }
}

#[derive(Clone)]
pub struct TransactionOutput {
    pub value: f64,
    pub address: Vec<u8>, // RSA public key in PEM format
}

impl TransactionOutput {
    pub fn new(value: f64, address: Vec<u8>) -> TransactionOutput {
        TransactionOutput { value, address }
    }
}

#[cfg(test)]
mod transaction_data_tests {
    use super::*;

    const prev_hash0: &'static str = "43c20c58a3dbfa0988f738868c7a64b2f3ba88d6d5b52065000576b0faa237fb";
    const prev_hash1: &'static str = "3ad9d0b19f13ff8d09db0c9a8236537a2c9ec01fef1ad9debb8dc46095e85ce9";
    const out_value0: f64 = 1.55;
    const out_value1: f64 = 3.05;
    const pem_pub_key0: &'static str = "-----BEGIN PUBLIC KEY----- \
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA2a7z3rWyTq21RKrYhWOt \
Lobjz+8UcV4lRgZwaFrGbjZfUaJ+pH7tx3pyjZZ0FNlbprlPauUgVzaVEWTshyAK \
4szDQRiMTT1sByGQVE6fQvL+PHR55jJ6lgaJ61vxfqaSA1Oy5Gn/aIWsptYHnVWa \
Z3rIqngVMAMwPxwK213sMmH+OousPITsF150c8ycKXQpQLiebQ/vLNo6Brjs/63C \
2rrh/TvkVA+P8mBJZZBm5CwRF4YU8qNBDFBC1xeqjQK+Asab7Wx2fCaai1HdZgZm \
QwoaxJbvXu4OsTOYIM81eNCJUp+HzQpFseqOkkIu2lCvz9e57TuJlYqOTk3Oc5+5 \
eQIDAQAB \
-----END PUBLIC KEY-----";
    const pem_priv_key0: &'static str = "-----BEGIN PRIVATE KEY----- \
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDZrvPetbJOrbVE \
qtiFY60uhuPP7xRxXiVGBnBoWsZuNl9Ron6kfu3HenKNlnQU2VumuU9q5SBXNpUR \
ZOyHIArizMNBGIxNPWwHIZBUTp9C8v48dHnmMnqWBonrW/F+ppIDU7Lkaf9ohaym \
1gedVZpnesiqeBUwAzA/HArbXewyYf46i6w8hOwXXnRzzJwpdClAuJ5tD+8s2joG \
uOz/rcLauuH9O+RUD4/yYEllkGbkLBEXhhTyo0EMUELXF6qNAr4CxpvtbHZ8JpqL \
Ud1mBmZDChrElu9e7g6xM5ggzzV40IlSn4fNCkWx6o6SQi7aUK/P17ntO4mVio5O \
Tc5zn7l5AgMBAAECggEBAMC2TPmWO4PB7t8arNLyGmg9TMNRsfRnV3I10x/fdRov \
EpWv2JQCNrlJYs6MKrombygbl/5XWBk9nhynD2rU6C4+/oDLnbHntZJemWq5q+7W \
NlMI/r3XZIUaxtDRNetcxZkiaRYj7NP4u497nYQhO69umOWpp4A44maMieQIs0kp \
qxp0fvxiJr6cEpDmfzbst0EPQwK92VbFjhOSqYh6YcNYW9FJhXiJOgbfkBMrraKE \
Dn7Eb3Vmzk3gZMMGud1nuJnsiZYQJ0J9O6Cm23RGCmB2OBo+gSTqH2Xfa64DhVjB \
fBVD4ancGky9dnJrLEt307ft8CdypHW9WzSAnSWVp10CgYEA+OiR+a8XbOkUq9Yv \
LOG7kBPeuj7rtnP5htx/T/Y/wFpZdhRd/PnFZjMpdTv17CsI/GsQ3BONIz6jDSaa \
PYXr4fARTaBP7laqdSeUBb5dgvrtaiy6AYtNlIaUMOm9Z8h+T5SrXczAi/i3F/+j \
WGWzQaB7WE2BbwnIbERb3Xbycr8CgYEA3+Kj7+gcFGBni05lOaw1TKBZ07jgCYZk \
YfSCogWs2W/dVlEx/auLXuSp4GUMND+Rw7L0A65poVY/uSvN2IeQwVFzgbYT8+pJ \
Zi03TzhbJxIetUtzoqx4c5DN+13ZSd0+egJK2E7CGjoB4FrZnalPo9BSBnMsBskn \
DWuqiJFkOccCgYEA5GY8tNmW7EhAwKFsbonAW6fwBAUCtExdVwP0CwLSYwZE+xYb \
XKwxF+OwkjPwKMMgnsb8FIYYR5QNeF7Iv9Woqo1ow0tsrS53gcNMj6ysECmDO59J \
G2uhR73qM6v4MkiGpy2rxgnBUW9rSyk30UCZKYpCRLfyIlrev4JGrcSdGu8CgYAq \
CTLAq7MB/GvUkx6caoIDZiQXhaHDCBG29qLEaw5eMQu81jftqhTb91ESCcb11G24 \
8dOxEUFSApCqYtgebK24LmPimd47MOUhIyI8ZJdwyY0sewM2Ku3evPR/2soefUTq \
wZ1t6bO2GugZSNWNLan/VvDftyYwC/qiIXH+fFX2JwKBgDpMb5+NVaffaHNxXe0D \
Ahh8FhfC9+RhB7BYb1gigkq4QrxFDBKmPiaNb56GOguB3f5cln/rovqi10AbmwhX \
JI2HurMm1jY7BMYanMD2jAnomx4OMgP3XFpowBCPqy33rcL7kJNQSK/Dp+uN+jXT \
/VJWcISkAebmWdUYZxKapMm0 \
-----END PRIVATE KEY-----";
    const pem_pub_key1: &'static str = "-----BEGIN PUBLIC KEY----- \
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAp91/IcqyUp9w6RMCBHmL \
9YDZ+Rfr5Cs/cbzQtadgYbKE83KphnWOXcA47bhCHRGORglvTQkMSlcIsLQ2+TAh \
BOTab8wOcWIKrVGRAdBXTwXGmFUmsu8C2gEc0I3GP6oRDL+UmLkmGi787CHcatu/ \
Pzsyi7cbmaPwYue6kWFGIz0yPoguqLKnmH32ERogTZAjvpR5vTPoFHJ1XQkhtbjk \
zeEg0UfFUxUJI7QTAvh+MNplbwPMvuA6JEKorbUzJY1I/JJcyzhvZOznRIYJBP3/ \
SqaeHcU5PbXj2TwaPZX+LCgNHClIPOnYA6qFraC8xLwZ8HdkFeMjVwFzN+gNtDEI \
2QIDAQAB \
-----END PUBLIC KEY-----";
    const pem_priv_key1: &'static str = "-----BEGIN PRIVATE KEY----- \
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCn3X8hyrJSn3Dp \
EwIEeYv1gNn5F+vkKz9xvNC1p2BhsoTzcqmGdY5dwDjtuEIdEY5GCW9NCQxKVwiw \
tDb5MCEE5NpvzA5xYgqtUZEB0FdPBcaYVSay7wLaARzQjcY/qhEMv5SYuSYaLvzs \
Idxq278/OzKLtxuZo/Bi57qRYUYjPTI+iC6osqeYffYRGiBNkCO+lHm9M+gUcnVd \
CSG1uOTN4SDRR8VTFQkjtBMC+H4w2mVvA8y+4DokQqittTMljUj8klzLOG9k7OdE \
hgkE/f9Kpp4dxTk9tePZPBo9lf4sKA0cKUg86dgDqoWtoLzEvBnwd2QV4yNXAXM3 \
6A20MQjZAgMBAAECggEATj9kZIcMqpDh2/NdTGKwB+dhM8ifz8MNFuJx00tLFR9W \
8/gt55ximXbh0oXCY7RsQl9hf2JJVVnOljfbLDrwGUzoOZa/4MBXw8SyuEq6d/50 \
PUvr9xqMlLxSBzEfCUwoGG4xQSOFDE183kFGcpUuR7Y7cH8RIYQWqLPl9qCGRiQe \
RIwX+VvfQyOouwraRMAEAJRZu6p0u2V4wtsV4rt0eSSvL+o/201UNdT9v35zPLgL \
VdWqW5mO/Qmut1IYUypofNU/l460ViKw0Ozaqqz1CXsl9D2dcW2iswQKJhklccRJ \
DjKlLH4ub7W/dccVH+Hjqs63D7CP4tCcx2ZdbkLbgQKBgQDQjwJgiWbWr6k3hX5c \
l/iF8PI/i9y2kMDgxOUh2Q7q2EFiLZKnhKfuCT//Ong7u55gY658bgfgjnn7p0Ru \
xWVyfE9CQ/5yzrbPgMwMDXSxUGeXLJw7KwhL2F16SPkyKMLB16/kWva0GRi0fV2S \
FDOFHax9bsiLUg1TkkKPlWV/6QKBgQDODMnwO701b4XqOWvNonMy+GTOzZvfVUzm \
7MNQPGK4Pz2lV/t6Bf4GYv0kzINrzahbCmcvxiyFm8dA2zn9eX+4vd+68qN7rcrW \
fwFL/FdanGFoEjS8SAhjyEPahaRvVvz9adWq2fxeA/4vhH5OGopHIpmtlw8zWxqf \
6TI+hPYbcQKBgQCDP2xUikOgmX1ZRnZGGRE6YW5iFUd50NDA4sf7rBiaLCvBeEKR \
j4cK4uFWYlpl5OV/bVvSTIBCjgcwGoyTVUBJcveCET4gy/v5y+kdMJ6eM6ZtWZKc \
HbGj4W66VRAVw9cEnBLCF4inwB9u/nITSwk9HXZ+nWgxXRqr3CBtMaxleQKBgFBi \
HtmwhIT3J/gTRKIpUOW+j506CygaX/DqxttjY2PbkBIT+9BDzDDzpywW+OIyjg5O \
RakWl8Hb3uTHYZ0oLBKHSGPnSq3yQ+JgE8JwBCgeP8XY7GfTdipvM0Fpx5eECRhX \
lHqdpd2Lkzs4ZgnUQsOzlN5qwwxW61EdKXPIbTaRAoGBALh+I0eNswMRmsR6a1zr \
6vv310P0AjBFLm2m5Duyw5wTXIzqrx4pGC4Vz1CbP62Xp+kbp44Ds6xzRYvoTqMA \
HAU72C5B3qV8ULoSZ9ispQa3gKuPrWuZtilm7sdc+C6g3Y5g5dBBfPszfa0AXeE+ \
yv1nbV05ly8kvVqOikizu06t \
-----END PRIVATE KEY-----";

    fn init_tx() -> Transaction {
        let mut tx = Transaction::new();
        //
        let id_0 = 0 as i32;
        let id_1 = 1 as i32;
        // add input txs
        tx.add_input_tx(TransactionInput::new(
            Vec::from_hex(prev_hash0).unwrap(),
            id_0,
        ));
        tx.add_input_tx(TransactionInput::new(
            Vec::from_hex(prev_hash1).unwrap(),
            id_1,
        ));
        // add output txs
        tx.add_output_tx(TransactionOutput::new(
            out_value0,
            pem_pub_key0.as_bytes().to_vec(),
        ));
        tx.add_output_tx(TransactionOutput::new(
            out_value1,
            pem_pub_key1.as_bytes().to_vec(),
        ));
        tx
    }

    #[test]
    fn raw_data_test() {
        let tx = init_tx();
        let raw_data_to_sigh_0 = tx.get_raw_data_to_sign(0);
        let raw_data_to_sigh_1 = tx.get_raw_data_to_sign(1);
        // check tx 0
        assert_eq!(
            Vec::from_hex(prev_hash0).unwrap(),
            raw_data_to_sigh_0[0..32].to_vec()
        );
        assert_eq!(
            Vec::from_hex("00000000").unwrap(),
            raw_data_to_sigh_0[32..36].to_vec()
        );
        unsafe {
            assert_eq!(
                mem::transmute::<&f64, &[u8; 8]>(&out_value0).to_vec(),
                raw_data_to_sigh_0[36..44].to_vec()
            );
        }
        // Skip checking public key
        unsafe {
            assert_eq!(
                mem::transmute::<&f64, &[u8; 8]>(&out_value1).to_vec(),
                raw_data_to_sigh_0[494..502].to_vec()
            );
        }
        // check tx 1
        assert_eq!(
            Vec::from_hex(prev_hash1).unwrap(),
            raw_data_to_sigh_1[0..32].to_vec()
        );
        assert_eq!(
            Vec::from_hex("01000000").unwrap(),
            raw_data_to_sigh_1[32..36].to_vec()
        );
        unsafe {
            assert_eq!(
                mem::transmute::<&f64, &[u8; 8]>(&out_value0).to_vec(),
                raw_data_to_sigh_1[36..44].to_vec()
            );
        }
        // Skip checking public key
        unsafe {
            assert_eq!(
                mem::transmute::<&f64, &[u8; 8]>(&out_value1).to_vec(),
                raw_data_to_sigh_1[494..502].to_vec()
            );
        }
    }
}
