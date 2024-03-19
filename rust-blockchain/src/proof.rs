use num_bigint::BigUint;
use sha2::{Digest, Sha256};
use crate::block::Block;
const DIFFICULTY: u64 = 18;
const MAX_NONCE: u64 = std::u64::MAX;

pub struct ProofOfWork<'a> {
    pub block: &'a Block,
    pub target: BigUint,
}


impl<'a> ProofOfWork<'a>{
    pub fn new_proof(block: &'a Block) -> Self{
        let mut target = BigUint::from(1u64);
        target = target << (256 - DIFFICULTY);
        return ProofOfWork{block, target};
    }
    pub fn init_data(&mut self, nonce : u64) -> Vec<u8> {
        let data = [self.block.data.clone(), self.block.prev_hash.clone(), to_hex(nonce), to_hex(DIFFICULTY)].concat();
        return data;
    }
    pub fn run(&mut self) -> (u64, Vec<u8>) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        let mut int_hash:BigUint;
        while nonce < MAX_NONCE {
            let data = self.init_data(nonce);
            let mut hasher = Sha256::new();
            hasher.update(&data);    //computes the hashed version of data.
            hash = hasher.finalize().to_vec();
            int_hash = BigUint::from_bytes_be(&hash);
            print!("Hash: {}\r", int_hash);
            if int_hash < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        return (nonce, hash);
    }
    pub fn validate(&mut self) -> bool {
        let int_hash:BigUint;
        let data = self.init_data(self.block.nonce);
        let mut hasher = Sha256::new();
        hasher.update(&data);    //computes the hashed version of data.
        let hash = hasher.finalize().to_vec();
        int_hash = BigUint::from_bytes_be(&hash);
        return int_hash < self.target;
    }
}
pub fn to_hex(num: u64) -> Vec<u8> {
    let hex = format!("{:x}", num);
    let hex = hex.as_bytes().to_vec();
    return hex;
}