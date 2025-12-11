use super::Block;
use crate::sha;

// proof of work: data + counter

const TARGET_BITS: u64 = 16;
const MAX_NONCE: u64 = 100000;

fn hash_to_int(hash_str: &str) -> u128 {
    // sha_256 returns a 64-char hex string (256 bits); clamp to 32 chars to fit u128
    let trimmed = hash_str.trim_start_matches("0x");
    let narrowed = if trimmed.len() > 32 {
        &trimmed[..32]
    } else {
        trimmed
    };
    u128::from_str_radix(narrowed, 16)
        .expect("ParseIntError: Error translating hash to uint")
}

#[derive(Debug)]
pub struct PoW {
    block: Block,
    target: u128,
}

impl PoW {

    pub fn new(block: Block) -> Self {
        // shift the max right by TARGET_BITS
        let target: u128 = u128::MAX >> (TARGET_BITS as u32);
        PoW { block, target }
    }

    fn pow_data(&self, nonce: u64) -> (String, String, u64) {
        let headers: (String, String, u64) = (
            self.block.prev_block_hash.clone(),
            self.block.data.clone(),
            self.block.time_stamp + TARGET_BITS + nonce,
        );
        headers
    }

    pub fn pow_done(&self) -> (u64, String) {
        
        let mut hash_int: u128 = 0;
        let mut nonce: u64 = 0;
        println!("Mining the block containing {} ", self.block.data); 

        while nonce < MAX_NONCE {
                let pow_hash: String = sha::sha_256(self.pow_data(nonce));
            hash_int = hash_to_int(&pow_hash);
            if hash_int < self.target {
                break;
            }
            nonce += 1;
        };
        (nonce, format!("0x{:032x}", hash_int))
    }

}
