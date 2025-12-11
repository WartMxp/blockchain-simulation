#[test]
fn test_sha_work() {
    use super::*;

    let _hash = String::new();
    let _data = String::new();
    let ts: u64 = 0;
    let hash_data = sha_256(_hash, _data);
    
    println!("{}", hash_data);
}