mod store;
#[cfg(test)]
mod tests;
mod sha;
mod pow;

use std::{process, time::SystemTime, io};
use crate::{pow::PoW, sha::sha_256};
use serde::{Serialize, Deserialize};
use store::{load_json, save_json};

pub const CHAIN_PATH: &str = "src/data/chain.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub time_stamp: u64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    fn new_genesis() -> Self {
        Block::new("Genesis Block", "0x0")
    }

    pub fn new<D, P>(data: D, prev_block_hash: P) -> Self
    where
        D: Into<String>,
        P: Into<String>,
    {
        let mut block = Block {
            time_stamp: 0,
            data: data.into(),
            prev_block_hash: prev_block_hash.into(),
            hash: String::new(),
            nonce: 0,
        };

        block.set_hash();
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

    // rerun pow
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
    let data = format!("The {}th block", chain.len());

    chain.push(Block::new(data, prev));
    chain
}

/// 若文件不存在则创建创世块并持久化
pub fn load_chain_or_init(path: &str) -> anyhow::Result<Vec<Block>> {
    match load_json(path) {
        Ok(chain) => Ok(chain),
        Err(err) => {
            if let Some(io_err) = err.downcast_ref::<io::Error>() {
                if io_err.kind() != io::ErrorKind::NotFound {
                    return Err(err);
                }
            }
            let chain = new_blockchain();
            save_json(path, &chain)?;
            Ok(chain)
        }
    }
}

/// 将当前链写回固态
pub fn persist_chain(path: &str, chain: &[Block]) -> anyhow::Result<()> {
    save_json(path, chain)
}
