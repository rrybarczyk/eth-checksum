extern crate sha3;

use sha3::{Digest, Keccak256};

fn main() {
    // Collect address from cmd line
    let args = std::env::args().collect::<Vec<String>>();
    let addr = args.get(1).unwrap();

    // Add addr validity checks

    // Remove "0x" prefix if exists and make everything lowercase
    let lc_addr = strip_0x(addr).to_lowercase();

    // keccak256 of address
    let haddr = hash_addr(&lc_addr);

    // Print final checksum
    println!("checksum: {}", checksum(&lc_addr, &haddr));
}

// Remove "0x" prefix
fn strip_0x(addr: &str) -> &str {
    if &addr[0..2] == "0x" {
        &addr[2..]
    } else {
        addr
    }
}

// Take keccak256 of formatted address
fn hash_addr(lc_addr: &str) -> String {
    // Declare hashing method
    let mut hasher = Keccak256::default();

    // Preimage is formatted address bytes
    hasher.input(lc_addr.as_bytes());

    // Get hash result
    let addr_hash = hasher.result();

    // Get vector of bytes in hex form
    let hex_vec = addr_hash.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>();

    // Join each hex byte representation in vector into single hex string and return
    hex_vec.join("")
}

fn checksum(addr: &str, addr_hash: &str) -> String {
    // Define new empty string to hold checksum addr
    let mut checksum_addr = String::new();

    for (c, hash_char) in addr.chars().zip(addr_hash.chars()) {
        // If ith hash char is greater than 8, capitilize ith addr char
        if hash_char.to_digit(15) > Some(8) {
            checksum_addr.extend(c.to_uppercase());
        } else {
            checksum_addr.push(c);
        }
    }

    checksum_addr
}
