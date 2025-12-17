use crate::{add_block_into_block_chain, load_chain_or_init, new_blockchain};
use std::fs;
use std::path::PathBuf;

#[test]
fn genesis_block_chain_has_one_block() {
    let blockchain = new_blockchain();
    assert_eq!(blockchain.len(), 1);
    let genesis = &blockchain[0];
    assert_eq!(genesis.prev_block_hash, "0x0");
}

#[test]
fn adding_block_increases_length_and_links_hash() {
    let mut chain = new_blockchain();
    let prev_hash = chain.last().unwrap().hash.clone();
    add_block_into_block_chain(&mut chain);
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[1].prev_block_hash, prev_hash);
}

#[test]
fn load_or_init_writes_when_missing() {
    // use a temp file path to avoid touching real data
    let mut path = PathBuf::from(std::env::temp_dir());
    path.push("blockchain_simulation_chain_test.json");

    // ensure absent
    let _ = fs::remove_file(&path);
    let chain = load_chain_or_init(path.to_str().unwrap()).expect("should create new chain");
    assert_eq!(chain.len(), 1);

    // now file should exist and be readable
    let chain2 = load_chain_or_init(path.to_str().unwrap()).expect("should load existing chain");
    assert_eq!(chain2.len(), 1);

    let _ = fs::remove_file(path);
}
