extern crate sha3;
extern crate regex;

use sha3::{Digest, Keccak256};
use regex::Regex;

fn main() {
    // Define address, should pass in via cmd
    let addr = "0x4B0897b0513fdC7C541B6d9D7E929C4e5364D2dB";
    // Add addr validity checks

    // Remove "0x" prefix if exists and make everything lowercase
    let lc_addr = strip_0x(addr).to_lowercase();

    // keccak256 of address
    let haddr = hash_addr(&lc_addr);

    // Print final checksum
    println!("checksum: {}", checksum(&lc_addr, &haddr));
}

// Remove "0x" prefix
fn strip_0x(addr: &str) -> String {
    let re = Regex::new(r"[0][x]").unwrap();
    return re.replace_all(addr, "").to_string();
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
    // Iterate for each byte, format each to hex string with map, collect into vector
    let hex_vec: Vec<String> = addr_hash.iter()
        .map(|b| format!("{:02X}", b))
        .collect();

    // Join each hex byte representation in vector into single hex string and return
    return hex_vec.join("");
}

fn checksum(addr: &str, addr_hash: &str) -> String {

    // Define new empty string to hold checksum addr
    let mut checksum_addr = String::new();

    // Access each character in addr hash
    let mut addr_hash_chars = addr_hash.chars();

    for (i, c) in addr.chars().enumerate() {
        // Get hash char to evaluate
        let mut hash_char = addr_hash_chars.next().unwrap();

        // If ith hash char is greater than 8, capitilize ith addr char
        if hash_char.to_digit(15) > Some(8) {
            checksum_addr.extend(c.to_uppercase());
        } else {
            checksum_addr.insert(i, c);
        }
    }
    return checksum_addr;
}
