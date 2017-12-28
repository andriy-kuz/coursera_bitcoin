use block::Block;
use transaction::Transaction;
use transaction_pool::TransactionPool;
use utxo::UTXOPool;

static CUT_OFF_AGE: usize = 10;
/// Block Chain should maintain only limited block nodes to satisfy the functions
/// You should not have all the blocks added to the block chain in memory
/// as it would cause a memory overflow.
/// Draft implementation
pub struct Blockchain {
    _utxo_pool: UTXOPool,
    _tx_pool: TransactionPool,
    _heights_block: Block,
}

impl Blockchain {
    pub fn new(genesis_block: Block) -> Self {
        Blockchain {
            _utxo_pool: UTXOPool::new(),
            _tx_pool: TransactionPool::new(),
            _heights_block: genesis_block,
        }
    }

    pub fn get_max_height_block(&self) -> &Block {
        &self._heights_block
    }

    pub fn get_max_height_utxo_pool(&mut self) -> &mut UTXOPool {
        &mut self._utxo_pool
    }

    pub fn get_max_height_tx_pool(&mut self) -> &mut TransactionPool {
        &mut self._tx_pool
    }

    pub fn add_block(&mut self, _: Block) -> bool {
        true
    }

    pub fn add_tx(&mut self, _: Transaction) {}
}
