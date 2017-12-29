use block::Block;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use time::Timespec;
use transaction_pool::TransactionPool;
use transaction::Transaction;
use utxo::UTXOPool;
use std::ops::DerefMut;

struct Branch {
    pub _blocks: Vec<Block>,
    pub _timestamp: Timespec,
    pub _utxo_pool: UTXOPool,
}

impl PartialEq for Branch {
    fn eq(&self, other: &Branch) -> bool {
        self._blocks.len() == other._blocks.len()
    }
}

impl Eq for Branch {}

impl PartialOrd for Branch {
    fn partial_cmp(&self, other: &Branch) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Branch {
    fn cmp(&self, other: &Branch) -> Ordering {
        // check brahches height and return max height branch
        // in case height equal - return oldest branch
        self._blocks
            .len()
            .cmp(&other._blocks.len())
            .then_with(|| other._timestamp.cmp(&self._timestamp))
    }
}

static CUT_OFF_AGE: usize = 10;
/// Block Chain should maintain only limited block nodes to satisfy the functions
/// You should not have all the blocks added to the block chain in memory
/// as it would cause a memory overflow.
/// Draft implementation
pub struct Blockchain {
    _tx_pool: TransactionPool,
    /// BinaryHeap hold all branches with blocks and
    /// timespec of creation of last block
    _branches: BinaryHeap<Branch>,
}

impl Blockchain {
    pub fn new(genesis_block: Block) -> Self {
        Blockchain {
            _tx_pool: TransactionPool::new(),
            _branches: BinaryHeap::new(),
        }
    }

    pub fn get_max_height_block(&self) -> &Block {
        &self._branches.peek().unwrap()._blocks.last().unwrap()
    }

    pub fn get_max_height_utxo_pool(&mut self) -> &mut UTXOPool {
        &mut self._branches.peek_mut().unwrap().deref_mut()._utxo_pool
    }

    pub fn get_max_height_tx_pool(&mut self) -> &mut TransactionPool {
        &mut self._tx_pool
    }

    pub fn add_block(&mut self, _: Block) -> bool {
        true
    }

    pub fn add_tx(&mut self, _: Transaction) {}
}
