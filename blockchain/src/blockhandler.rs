use block::Block;
use blockchain::Blockchain;
use transaction::Transaction;
use txhandler::TxHandler;

pub struct BlockHandler {
    blockchain: Blockchain,
}

impl BlockHandler {
    pub fn new(blockchain: Blockchain) -> Self {
        BlockHandler { blockchain }
    }

    pub fn process_block(&mut self, block: Block) -> bool {
        self.blockchain.add_block(block)
    }

    pub fn precess_tx(&mut self, tx: Transaction) {
        self.blockchain.add_tx(tx);
    }

    pub fn create_block(&mut self, my_address: Vec<u8>) -> Block {
        let parent_hash = self.blockchain.get_max_height_block().hash().clone();
        let mut current = Block::new(parent_hash, my_address);
        let mut txs = self.blockchain.get_max_height_tx_pool().get_all_txs();
        {
            //TODO: txpoll changes, but not involved in heap reorganization
            let mut branch = self.blockchain.get_max_height_branch();
            let mut tx_handler = TxHandler::new(&mut branch._utxo_pool);
            txs = tx_handler.handle_txs(txs);
        }

        for tx in txs {
            current.add_tx(tx);
        }
        current.finalize();
        self.blockchain.add_block(current);
        //
        Block::new([0; 32], Vec::new())
    }
}
