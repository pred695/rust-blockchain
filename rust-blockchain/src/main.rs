use sha2::{Digest, Sha256};
struct Block {
    hash: Vec<u8>,
    data: Vec<u8>,
    prev_hash: Vec<u8>,
}

impl Block {
    fn derive_hash(&mut self) {
        let info = [self.data.clone(), self.prev_hash.clone()].concat();
        let mut hasher = Sha256::new();
        hasher.update(&info);
        self.hash = hasher.finalize().to_vec();
    }
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn add_block(&mut self, data: Vec<u8>) {
        let prev_hash = self.blocks.last().unwrap().hash.clone();
        let mut new_block = Block {
            data,
            prev_hash,
            hash: Vec::new(),
        };
        new_block.derive_hash();
        self.blocks.push(new_block);
    }
    fn init_blockchain(&mut self) {
        let genesis_block = genesis();
        self.blocks.push(genesis_block);
    }
}

fn genesis() -> Block {
    Block {
        hash: vec![0; 32],
        data: "First block".as_bytes().to_vec(),
        prev_hash: Vec::new(),
    }
}

fn main() {
    let mut chain = Blockchain { blocks: Vec::new() };
    chain.init_blockchain();
    chain.blocks[0].derive_hash();  //deriving the hash of the genesis block
    chain.add_block("Second block".as_bytes().to_vec());
    chain.add_block("Third block".as_bytes().to_vec());
    chain.add_block("Fourth block".as_bytes().to_vec());
    println!("{:?}", chain.blocks[0].hash);
    for block in &chain.blocks {
        let hash = hex::encode(&block.hash);
        println!("Hash: {}", hash);
        let data = String::from_utf8(block.data.clone()).unwrap();
        println!("Data: {}", data);
        let prev_hash = hex::encode(&block.prev_hash);  //Using hex crate here instead of String::from_utf8 because the sha256 results in a binary 
        println!("Prev. hash: {}", prev_hash);
        println!("")
    }
}
