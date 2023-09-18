extern crate blake2;
extern crate hex;

use blake2::{Blake2b512, Blake2s256,Blake2b, Digest};

fn blake2b512_hash(data: &[u8]) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hex::encode(hash)
}

fn blake2s256_hash(data: &[u8]) -> String {
    let mut hasher = Blake2s256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hex::encode(hash)
}

fn main() {
    // create a Blake2b512 object
    // Data to be hashed
    let data = "Shree Ganeshaya Namah!".as_bytes();
    let data1 = "JaiShree Ram!".as_bytes();
    let data2 = "Jai BajarangaBalai!".as_bytes();

    // Calculate BLAKE2b512 hash
    let hash_2b512 = blake2b512_hash(data);

    // Calculate BLAKE2s256 hash
    let hash_2s256 = blake2s256_hash(data2);

    println!("BLAKE2b512 Hash (Hex): {}", hash_2b512);
    println!("BLAKE2s256 Hash (Hex): {}", hash_2s256);

}
