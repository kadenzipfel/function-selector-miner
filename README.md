## About

Blazingly fast function selector miner written in Rust.

## Usage

```
git clone git@github.com:kadenzipfel/function-selector-miner.git
cd function-selector-miner
cargo run <function name> <function params> <leading zeroes>
```

e.g. `cargo run "cat" "(address,uint256)" 3` outputs:
```
Function selector: 0000002e, Signature: cat7826901(address,uint256)
Time taken: 12.12071 seconds
```
