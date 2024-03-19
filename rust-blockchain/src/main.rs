mod block;
mod blockchain;
mod proof;
use blockchain::Blockchain;
fn main() {
    let mut chain = Blockchain { blocks: Vec::new() };
    chain.init_blockchain();
    chain.add_block("Second block".as_bytes().to_vec());
    chain.add_block("Third block".as_bytes().to_vec());
    chain.add_block("Fourth block".as_bytes().to_vec());
    for block in &chain.blocks {
        let data = String::from_utf8(block.data.clone()).unwrap();
        println!("Data: {}", data);
        let hash = hex::encode(&block.hash);
        println!("Hash: {}", hash);
        let prev_hash = hex::encode(&block.prev_hash);  //Using hex crate here instead of String::from_utf8 because the sha256 results in a binary 
        println!("Prev. hash: {}", prev_hash);
        //Proof of work
        let mut pow = proof::ProofOfWork::new_proof(&block);
        println!("PoW: {}", pow.validate());
        println!("")
    }
}
