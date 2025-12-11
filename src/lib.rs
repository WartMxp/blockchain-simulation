mod sha;
mod pow;

use std::{ process, time::SystemTime };
use crate::{pow::PoW, sha::sha_256};

#[derive(Debug, Clone)]
pub struct Block {
    pub time_stamp: u64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {

    fn new_genesis() -> Self {
        Block::new("Genesis Block".to_string(), String::new())
    }

    pub fn new(data: String, prev_block_hash: String) -> Self {
        let mut block = Block {
            time_stamp: 0,
            data,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
        };

        // set timestamp before mining so PoW can use it
        block.set_hash();

        // create PoW from a clone so we can keep ownership of `block`
        let pow = PoW::new(block.clone());
        let (nonce, hash) = pow.pow_done();
        block.nonce = nonce;
        block.hash = hash;

        block
    }

    fn set_hash(&mut self) {
        
        let time_stamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_else(|err| {
                eprintln!("Error getting time: {}", err);
                process::exit(1);
            });
        // store the timestamp into the block
        self.time_stamp = time_stamp;
        let headers: (String, String, u64) = (
            self.prev_block_hash.clone(),
            self.data.clone(),
            time_stamp,
        );
        self.hash = sha_256(headers);

    }
}

pub fn print_block(block: &Block) {
    println!("----------------------------------------");
    println!("Timestamp: {}", block.time_stamp);
    println!("Data: {}", block.data);
    println!("Prev hash: {}", block.prev_block_hash);
    println!("Stored nonce: {}", block.nonce);
    println!("Stored hash: {}", block.hash);

    // Re-run proof of work to demonstrate the interface.
    let pow = PoW::new(block.clone());
    let (nonce, hash) = pow.pow_done();
    println!("PoW result -> nonce: {}, hash: {}", nonce, hash);
}

pub fn new_blockchain() -> Vec<Block> {
    vec![Block::new_genesis()]
}

pub fn add_block_into_block_chain(chain: &mut Vec<Block>) -> &mut Vec<Block> {

    let prev = chain
        .get(chain.len() - 1)
        .unwrap()
        .hash
        .clone();
    let introduce: String = String::from(" normal block ");
    chain.push(Block::new(introduce, prev));
    chain
}

