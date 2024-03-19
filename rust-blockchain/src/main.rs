mod block;
mod blockchain;
use blockchain::Blockchain;
fn main() {
    let mut chain = Blockchain { blocks: Vec::new() };
    chain.init_blockchain();
    chain.blocks[0].derive_hash();  //deriving the hash of the genesis block
    chain.add_block("Second block".as_bytes().to_vec());
    chain.add_block("Third block".as_bytes().to_vec());
    chain.add_block("Fourth block".as_bytes().to_vec());
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
