use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::Path,
};

pub fn save_json<P, T>(path: P, value: &T) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    T: Serialize + ?Sized,
{
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}

pub fn load_json<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
