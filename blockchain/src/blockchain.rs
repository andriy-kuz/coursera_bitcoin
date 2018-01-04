use block::Block;
use std::cmp::Ordering;
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;
use std::collections::LinkedList;
use time;
use time::Timespec;
use transaction_pool::TransactionPool;
use transaction::Transaction;
use utxo::UTXOPool;

pub struct Branch {
    /// Blocks list, keeps only CUT_OFF_AGE number of newest blocks
    _blocks: LinkedList<Block>,
    /// Timestamp for last block in list
    _timestamp: Timespec,
    /// unspended transactions pool for this branch
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
    /// Global transaction pool
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
        &self._branches.peek().unwrap()._blocks.back().unwrap()
    }

    pub fn get_max_height_branch(&mut self) -> PeekMut<Branch> {
        self._branches.peek_mut().unwrap()
    }

    pub fn get_max_height_tx_pool(&mut self) -> &mut TransactionPool {
        &mut self._tx_pool
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        if *block.prev_hash() == [0; 32] {
            return false;
        }
        let mut branch = self._branches.peek_mut().unwrap();

        if *branch._blocks.back().unwrap().hash() != *block.prev_hash() {
            return false;
        }
        //TODO: heap must takes into account changes
        if branch._blocks.len() > CUT_OFF_AGE {
            branch._blocks.pop_front();
        }

        branch._blocks.push_back(block);
        branch._timestamp = time::get_time();
        true
    }

    pub fn add_tx(&mut self, tx: Transaction) {
        self._tx_pool.add_tx(tx)
    }
}
