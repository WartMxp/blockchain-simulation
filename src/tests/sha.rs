use crate::sha::sha_256;

#[test]
fn sha_is_deterministic() {
    let input = ("prev".to_string(), "data".to_string(), 123);
    let h1 = sha_256(input.clone());
    let h2 = sha_256(input);
    assert!(!h1.is_empty());
    assert_eq!(h1, h2);
}
