extern crate openssl;

use self::openssl::sign::Verifier;
use self::openssl::pkey::PKey;
use self::openssl::hash::MessageDigest;


pub fn verify_signature(pub_key: &Vec<u8>, msg: &Vec<u8>, sig: &Vec<u8>) -> bool {
    let pub_key = PKey::public_key_from_pem(pub_key).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pub_key).unwrap();
    verifier.update(msg).unwrap();
    verifier.finish(sig).unwrap()
}
