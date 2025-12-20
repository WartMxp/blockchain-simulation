# bs




A blockchain simulation system based on Rust and Satoshi Nakamoto's white paper.



## Quick Start


It based on stable-x86_64-pc-windows-gnu, so you don't need to install Visual C++ Redistributable.

```powershell
> RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
> blockchain-simulation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running 'target\debug\blockchain-simulation.exe'
Enter command: addblock -data "..." | printchain | exit
```


If you would like to add transaction data, you can do so as follows:

```powershell
> addblock -data "Amount 0.00714281BTC Fee 4,620SATS From 6 Inputs To bc1qs-wn28x"
```

