// Import the necessary modules
use std::env;
use std::time::Instant;
use tiny_keccak::{Hasher, Keccak};

// Function to calculate the Keccak-256 hash
fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(input);
    keccak.finalize(&mut output);
    output
}

// Function to mine the function selector
fn mine_function_selector(
    name: &str,
    params: &str,
    leading_zeros: usize,
) -> Option<(String, String)> {
    let mut nonce = 0;
    loop {
        let input = format!("{}{}{}", name, nonce, params);
        let hash = keccak256(input.as_bytes());
        let selector = &hash[..4];

        if selector.iter().take(leading_zeros).all(|&byte| byte == 0) {
            return Some((hex::encode(selector), input));
        }

        if nonce == std::u32::MAX {
            break;
        }

        nonce += 1;
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: <function name> <function params> <leading_zeros>");
        return;
    }

    let function_name = &args[1];
    let params = &args[2];
    let leading_zeros: usize = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid leading_zeros value");
            return;
        }
    };

    let start_time = Instant::now();

    match mine_function_selector(function_name, params, leading_zeros) {
        Some((selector, signature)) => {
            let elapsed_time = start_time.elapsed();
            println!("Function selector: {}, Signature: {}", selector, signature);
            println!(
                "Time taken: {}.{} seconds",
                elapsed_time.as_secs(),
                elapsed_time.as_millis()
            );
        }
        None => println!("No function selector found"),
    }
}
