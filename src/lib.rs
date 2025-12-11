mod sha;

use std::{ process, time::SystemTime };

use crate::sha::sha_256;

#[derive(Debug)]
struct Block {
    time_stamp: u64,
    data: String,
    prev_block_hash: String,
    hash: String,
}

impl Block {

    fn new_block(data: String, prev_block_hash: String) -> Self {
        
        let mut block = Block {
            time_stamp: 0,
            data,
            prev_block_hash,
            hash: String::new(),
        };
        block.set_hash();
        
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
        let headers: (String, String, u64) = (
            self.prev_block_hash.clone(),
            self.data.clone(),
            time_stamp,
        );
        self.hash = sha_256(headers);

    }
}