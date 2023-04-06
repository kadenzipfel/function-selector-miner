## About

A quick and dirty function selector miner written in Rust.

## Usage

```
git clone git@github.com:kadenzipfel/func-sel-miner.git
cd func-sel-miner
cargo run <function name> <function params> <leading zeroes>
```

e.g. `cargo run "cat" "(address,uint256)" 2` outputs `Function selector: 0000bd80, Signature: cat69753(address,uint256)`