use sha3::{Sha3_512, Digest};
use std::env;
use std::io::{self, Read};

fn main() {
    // Get the input from the first command-line argument if provided,
    // otherwise read from STDIN.
    let input = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from STDIN");
        buffer
    };

    // Compute the SHA3-512 hash.
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_bytes());
    let hash_result = hasher.finalize();

    // Print the hexadecimal representation of the hash.
    println!("{}", hex::encode(hash_result));
}


