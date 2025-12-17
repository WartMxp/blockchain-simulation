use blockchain_simulation::{
    Block, print_block, load_chain_or_init, persist_chain, CHAIN_PATH,
};

fn main() -> anyhow::Result<()> {
    let mut blockchain = load_chain_or_init(CHAIN_PATH)?;

    let prev = blockchain
        .last()
        .map(|b| b.hash.clone())
        .unwrap_or_else(|| "0x0".to_string());

    blockchain.push(Block::new("Normal block", prev));
    persist_chain(CHAIN_PATH, &blockchain)?;

    for block in blockchain.iter() {
        print_block(block);
    }
    Ok(())
}


