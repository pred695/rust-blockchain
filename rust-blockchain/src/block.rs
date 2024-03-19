use sha2::{Digest, Sha256};
pub struct Block {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub prev_hash: Vec<u8>,
}

impl Block {
    pub fn derive_hash(&mut self) {
        let info = [self.data.clone(), self.prev_hash.clone()].concat();
        let mut hasher = Sha256::new();
        hasher.update(&info);
        self.hash = hasher.finalize().to_vec();
    }
}


pub fn genesis() -> Block {
    Block {
        hash: vec![0; 32],
        data: "First block".as_bytes().to_vec(),
        prev_hash: Vec::new(),
    }
}