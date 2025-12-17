mod store;
#[cfg(test)]
mod tests;
mod sha;
mod pow;

use std::{process, time::SystemTime, io};
use std::env;
use crate::{pow::PoW, sha::sha_256};
use serde::{Serialize, Deserialize};
use store::{load_json, save_json};

pub const CHAIN_PATH: &str = "src/data/chain.json";

/// CLI
pub enum CliCommand {
    AddBlock { data: String },
    PrintChain,
}

/// 从参数列表解析命令；-data 后面的所有内容被拼成 data
fn parse_cli_args(args: impl IntoIterator<Item = String>) -> anyhow::Result<CliCommand> {
    let args: Vec<String> = args.into_iter().collect();

    // 命令可能在 0（REPL）或 1（普通 CLI，0 是二进制名）
    let cmd_idx = match args.get(0).map(|s| s.as_str()) {
        Some("addblock") | Some("printchain") => 0,
        _ => 1,
    };
    let cmd = args.get(cmd_idx).ok_or_else(|| anyhow::anyhow!(
        "Usage:\n  addblock -data \"Send 1 BTC to Ivan\"\n  printchain"
    ))?;
    let rest = &args[(cmd_idx + 1)..];

    match cmd.as_str() {
        "addblock" => {
            let data_pos = rest.iter().position(|s| s == "-data")
                .ok_or_else(|| anyhow::anyhow!("missing -data <payload>"))?;
            let data_tokens = &rest[(data_pos + 1)..];
            let data = data_tokens.join(" ").trim_matches('"').to_string();
            if data.is_empty() {
                return Err(anyhow::anyhow!("missing -data <payload>"));
            }
            Ok(CliCommand::AddBlock { data })
        }
        "printchain" => Ok(CliCommand::PrintChain),
        _ => Err(anyhow::anyhow!(
            "Usage:\n  addblock -data \"Send 1 BTC to Ivan\"\n  printchain"
        )),
    }
}

/// 执行解析后的命令
fn execute_command(cmd: CliCommand) -> anyhow::Result<()> {
    match cmd {
        CliCommand::AddBlock { data } => {
            let mut chain = load_chain_or_init(CHAIN_PATH)?;
            let prev_hash = chain
                .last()
                .map(|b| b.hash.clone())
                .unwrap_or_else(|| "0x0".to_string());
            chain.push(Block::new(data, prev_hash));
            persist_chain(CHAIN_PATH, &chain)?;
            println!("Block added. Chain length: {}", chain.len());
        }
        CliCommand::PrintChain => {
            let chain = load_chain_or_init(CHAIN_PATH)?;
            for block in chain.iter() {
                print_block(block);
            }
        }
    }
    Ok(())
}

/// CLI_START
pub fn run_cli() -> anyhow::Result<()> {
    let cmd = parse_cli_args(env::args())?;
    execute_command(cmd)
}

/// 简易交互
pub fn run_repl() -> anyhow::Result<()> {
    println!("Enter command: addblock -data \"...\" | printchain | exit");
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        let n = stdin.read_line(&mut line)?;
        if n == 0 {
            break; // EOF
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }
        let tokens: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
        match parse_cli_args(tokens) {
            Ok(cmd) => {
                if let Err(err) = execute_command(cmd) {
                    eprintln!("Error: {}", err);
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(())
}

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
