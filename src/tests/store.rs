use std::fs;
use std::path::PathBuf;
use crate::store::{load_json, save_json};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Dummy {
    x: u32,
    y: String,
}

#[test]
fn save_and_load_round_trip() {
    let tmp_dir = std::env::temp_dir();
    let mut path = PathBuf::from(tmp_dir);
    path.push("blockchain_simulation_test_roundtrip.json");

    let value = Dummy { x: 7, y: "hi".into() };
    save_json(&path, &value).expect("save_json should succeed");

    let loaded: Dummy = load_json(&path).expect("load_json should succeed");
    assert_eq!(loaded, value);

    // cleanup best-effort
    let _ = fs::remove_file(path);
}
