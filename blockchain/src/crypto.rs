use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sha::sha256;
use openssl::sign::Verifier;


pub fn verify_signature(pub_key: &Vec<u8>, msg: &[u8; 32], sig: &[u8; 32]) -> bool {
    let pub_key = PKey::public_key_from_pem(pub_key).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pub_key).unwrap();
    verifier.update(msg).unwrap();
    verifier.finish(sig).unwrap()
}

pub fn double_sha256(data: &Vec<u8>) -> [u8; 32] {
    let data = sha256(data).to_vec();
    sha256(&data)
}
