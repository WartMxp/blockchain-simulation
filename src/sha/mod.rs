pub(crate) mod sha;
pub(crate) mod utils;

use sha::{ SHA256Context };

pub fn sha_256((hash, _data, ts): 
    (String, String, u64)) -> String {
    
    let input = format!("{}{}{}", hash, _data, ts);
    let data_byte = input.as_bytes();

    let mut ctx = SHA256Context {
        buf: [0; 64],
        hash: [0; 8],
        bits: [0; 2],
        len: 0,
        rfu__: 0,
        w: [0; 64],
    };
    
    ctx.init();
    ctx.shash(data_byte);
    let mut _hash = ctx.hash.clone();
    ctx.sdone(&mut _hash)
    
}