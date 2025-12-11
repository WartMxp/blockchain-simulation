use blockchain_simulation::{Block, new_blockchain, print_block};

fn main() {

    let mut blockchain = new_blockchain();
    assert_eq!(blockchain.len(), 1);

    blockchain.push(Block::new(" normal block ".to_string(), blockchain
        .get(0)
        .unwrap()
        .hash
        .clone()
    ));
    for block in blockchain.iter() {
        print_block(block);
    }
}


