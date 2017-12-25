use block::Block;
use transaction::Transaction;
use transaction_pool::TransactionPool;
use utxo::UTXOPool;

static CUT_OFF_AGE: usize = 10;
/// Block Chain should maintain only limited block nodes to satisfy the functions
/// You should not have all the blocks added to the block chain in memory
/// as it would cause a memory overflow.
pub struct Blockchain {}

impl Blockchain {
    pub fn new(_: Block) -> Self {
        Blockchain {}
    }

    pub fn get_max_height_block(&self) -> Block {
        Block::new([0; 32], Vec::new())
    }

    pub fn get_max_height_utxo_pool(&self) -> UTXOPool {
        UTXOPool::new()
    }

    pub fn get_max_height_tx_pool(&self) -> TransactionPool {
        TransactionPool::new()
    }

    pub fn add_block(&mut self, _: Block) -> bool {
        true
    }

    pub fn add_tx(&mut self, _: Transaction) {}
}