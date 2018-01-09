use block::Block;
use std::cmp;
use std::cmp::Ordering;
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;
use std::collections::LinkedList;
use time;
use time::Timespec;
use transaction_pool::TransactionPool;
use transaction::Transaction;
use utxo::UTXOPool;
use utxo::UTXO;

pub struct Branch {
    _blocks: LinkedList<Block>,
    /// Timestamp for last block in list
    _timestamp: Timespec,
    /// unspended transactions pool for this branch
    pub _utxo_pool: UTXOPool,
}

impl Branch {
    fn new(genesis_block: Block) -> Self {
        let mut branch = Branch {
            _blocks: LinkedList::new(),
            _timestamp: time::get_time(),
            _utxo_pool: UTXOPool::new(),
        };
        // Add coinbase tx to utxo pool
        for (index, tx) in genesis_block.coinbase().get_outputs().iter().enumerate() {
            let utxo = UTXO::new(genesis_block.coinbase().hash().clone(), index);
            branch._utxo_pool.add_UTXO(utxo, tx.clone());
        }
        branch._blocks.push_back(genesis_block);
        branch
    }
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
        let mut blockchain = Blockchain {
            _tx_pool: TransactionPool::new(),
            _branches: BinaryHeap::new(),
        };
        blockchain._branches.push(Branch::new(genesis_block));
        blockchain
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
        // try add block to highest branch
        let highest_branch_len;
        {
            let mut branch = self._branches.peek_mut().unwrap();
            highest_branch_len = branch._blocks.len();

            if *branch._blocks.back().unwrap().hash() != *block.prev_hash() {
                Self::add_block_to_branch(&mut branch, block);
                return true;
            }
        }
        // search branch
        if self._branches
            .iter()
            .find(|ref branch| *branch._blocks.back().unwrap().hash() == *block.prev_hash())
            == None
        {
            return false;
        }
        // drain branches heap, add block to branch, recreate heap
        let mut new_branches = BinaryHeap::new();

        for mut branch in self._branches.drain() {
            if *branch._blocks.back().unwrap().hash() == *block.prev_hash() {
                Self::add_block_to_branch(&mut branch, block);
            }

            if cmp::max(highest_branch_len, branch._blocks.len())
                - cmp::min(highest_branch_len, branch._blocks.len()) < CUT_OFF_AGE
            {
                new_branches.push(branch);
            }
        }
        self._branches = new_branches;
        true
    }

    pub fn add_tx(&mut self, tx: Transaction) {
        self._tx_pool.add_tx(tx)
    }

    fn add_block_to_branch(branch: &mut Branch, block: Block) {
        // Add coinbase tx to utxo pool
        for (index, tx) in block.coinbase().get_outputs().iter().enumerate() {
            let utxo = UTXO::new(block.coinbase().hash().clone(), index);
            branch._utxo_pool.add_UTXO(utxo, tx.clone());
        }
        // add block
        branch._blocks.push_back(block);
        branch._timestamp = time::get_time();
    }
}
