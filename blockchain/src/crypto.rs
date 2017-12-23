use openssl::sign::Verifier;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use openssl::sha::sha256;


pub fn verify_signature(pub_key: &Vec<u8>, msg: &Vec<u8>, sig: &Vec<u8>) -> bool {
    let pub_key = PKey::public_key_from_pem(pub_key).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pub_key).unwrap();
    verifier.update(msg).unwrap();
    verifier.finish(sig).unwrap()
}

pub fn double_sha256(data: &Vec<u8>) -> Vec<u8> {
    let data = sha256(data).to_vec();
    sha256(&data).to_vec()
}
