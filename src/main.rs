use blockchain_simulation::{run_cli, run_repl};
use std::env;

fn main() -> anyhow::Result<()> {
    // 如果有参数则直接解析；否则进入交互模式
    if env::args().len() > 1 {
        run_cli()
    } else {
        run_repl()
    }
}
