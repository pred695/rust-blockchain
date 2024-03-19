use crate::block::{Block, genesis};
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn add_block(&mut self, data: Vec<u8>) {
        let prev_hash = self.blocks.last().unwrap().hash.clone();
        let mut new_block = Block {
            data,
            prev_hash,
            hash: Vec::new(),
        };
        new_block.derive_hash();
        self.blocks.push(new_block);
    }
    pub fn init_blockchain(&mut self) {
        let genesis_block = genesis();
        self.blocks.push(genesis_block);
    }
}
