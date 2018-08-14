extern crate sha3;

use sha3::{Digest, Keccak256};

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
//
// NB. Returning a &str here avoids an extra allocation.
//     The returned string will be a substring of the input.
//
// NB. The previous regex was `Regex::new(r"[0][x]")`,
//     but Regex::new("0x") would have had the same effect.
//     However, both were incorrect. Rust regexes match
//     anywhere in a string, you would need `^0x` to only
//     match the beginning of the string.
fn strip_0x(addr: &str) -> &str {
    // NB. explicit returns aren't needed if the value you
    //     wish to return is the last statment of the function
    //     or closure
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
    // Iterate for each byte, format each to hex string with map, collect into vector
    //
    // NB. Switched from type annotation to turbofish.
    //     Both are the same here, but sometimes a
    //     turbofish (which definitely looks like a
    //     fish: ::<>) is useful, since you can chain
    //     them.
    let hex_vec = addr_hash.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>();

    // Join each hex byte representation in vector into single hex string and return
    //
    // NB. Removed explicit return.
    hex_vec.join("")
}

fn checksum(addr: &str, addr_hash: &str) -> String {
    // Define new empty string to hold checksum addr
    let mut checksum_addr = String::new();

    // NB. Switched to zip of both char iterators
    for (c, hash_char) in addr.chars().zip(addr_hash.chars()) {
        // If ith hash char is greater than 8, capitilize ith addr char
        if hash_char.to_digit(15) > Some(8) {
            checksum_addr.extend(c.to_uppercase());
        } else {
            // NB. Changed to `push` which appends to the end of the String
            checksum_addr.push(c);
        }
    }

    // NB. removed explicit return
    checksum_addr
}
