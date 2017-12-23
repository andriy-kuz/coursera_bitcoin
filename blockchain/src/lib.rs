#![feature(const_size_of)]
#[macro_use]
extern crate bytevec;
extern crate openssl;
mod crypto;
mod transaction;
mod transaction_pool;
mod utxo;
