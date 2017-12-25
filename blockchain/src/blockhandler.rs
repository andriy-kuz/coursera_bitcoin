use blockchain::Blockchain;
use block::Block;
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
        let parent = self.blockchain.get_max_height_block();
        let parent_hash = parent.hash().clone();

        let mut current = Block::new(parent_hash, my_address);
        let utxo_pool = self.blockchain.get_max_height_utxo_pool();
        let tx_pool = self.blockchain.get_max_height_tx_pool();
        let mut tx_handler = TxHandler::new(utxo_pool);
        let txs = tx_pool.get_all_txs();
        let txs = tx_handler.handle_txs(txs);

        for tx in txs {
            current.add_tx(tx);
        }
        current.finalize();
        self.blockchain.add_block(current);
        //
        Block::new([0; 32], Vec::new())
    }
}