use crate::block::Block;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn add_block(&mut self, data: Vec<u8>) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let data = String::from_utf8(data).unwrap();
        let new_block = Block::create_block(data, prev_block.hash.clone());//2) Use the create block function
        self.blocks.push(new_block);
    }
    pub fn init_blockchain(&mut self) {
        let genesis_block = Block::genesis();
        self.blocks.push(genesis_block);
    }
}