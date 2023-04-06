// Import the necessary modules
use rayon::prelude::*;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tiny_keccak::{Hasher, Keccak};

const MAX_NONCE: u32 = std::u32::MAX;

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
    nonce_start: u32,
    nonce_step: u32,
    found: &AtomicBool,
) -> Option<(u32, String, String)> {
    let mut nonce = nonce_start;
    while nonce < MAX_NONCE && !found.load(Ordering::Relaxed) {
        let input = format!("{}{}{}", name, nonce, params);
        let hash = keccak256(input.as_bytes());
        let selector = &hash[..4];

        if selector.iter().take(leading_zeros).all(|&byte| byte == 0) {
            found.store(true, Ordering::Relaxed);
            return Some((nonce, hex::encode(selector), input));
        }

        nonce += nonce_step;
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

    let num_threads = num_cpus::get();
    let start_time = Instant::now();
    let found = AtomicBool::new(false);

    let results: Vec<Option<(u32, String, String)>> = (0..num_threads)
        .into_par_iter()
        .map(|i| {
            let nonce_start = i as u32;
            let nonce_step = num_threads as u32;
            mine_function_selector(
                function_name,
                params,
                leading_zeros,
                nonce_start,
                nonce_step,
                &found,
            )
        })
        .collect();

    let result = results.into_iter().filter_map(|r| r).min_by_key(|r| r.0);

    match result {
        Some((_nonce, selector, signature)) => {
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
