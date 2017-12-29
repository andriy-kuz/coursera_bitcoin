#![feature(const_size_of)]
extern crate openssl;
extern crate time;

mod block;
mod blockchain;
mod blockhandler;
mod crypto;
mod transaction;
mod transaction_pool;
mod txhandler;
mod utxo;
