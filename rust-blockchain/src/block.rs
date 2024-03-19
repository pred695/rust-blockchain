use crate::proof;
pub struct Block {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub prev_hash: Vec<u8>,
    pub nonce: u64,
}

impl Block {
    pub fn create_block(data: String, prev_hash: Vec<u8>) -> Self {
        let mut block = Block{
            hash: Vec::new(),
            data: data.as_bytes().to_vec(),
            prev_hash,
            nonce: 0,
        };
        let mut proof = proof::ProofOfWork::new_proof(&block);
        let (nonce, hash) = proof.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }
    pub fn genesis() -> Self {
        let data = String::from("Genesis block");
        let prev_hash = Vec::new();
        let genesis_block = Self::create_block(data, prev_hash);//3) Use the create function to create genesis block
        return genesis_block;
     }
     
}



