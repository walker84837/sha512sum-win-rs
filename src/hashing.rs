use openssl::hash::{Hasher, MessageDigest};
use std::fmt::Write;

pub struct Sha512Sum;

impl Sha512Sum {
    pub fn get_checksum(data: &[u8]) -> String {
        let mut hasher = Hasher::new(MessageDigest::sha512())
            .expect("Failed to create a digest");

        hasher.update(data).expect("Failed to make bytes into digest");
        let result = hasher.finish().expect("Failed to generate checksum");

        let mut checksum = String::with_capacity(result.len() * 2);

        // Use fold and write! macro to format each byte
        result.iter().fold(&mut checksum, |acc, &byte| {
            write!(acc, "{:02x}", byte).expect("Failed to write to string");
            acc
        });

        checksum
    }
}
