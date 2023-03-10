use std::time::Instant;

use hex;
use rayon::prelude::*;
use sha2::{Digest, Sha512_224};

fn main() {
    let times: usize = 300;
    let start = Instant::now();
    // (1..1000).par_bridge().for_each(|num| {
    //     calculate_hash(num.to_string(), times);
    // }); 21 ms
    // (1..1000).into_par_iter().for_each(|num| {
    //     calculate_hash(num.to_string(), times);
    // }); 22 ms
    // (1..1000).into_iter().for_each(|num| {
    //     calculate_hash(num.to_string(), times);
    // }); 150 ms
    let end = Instant::now();
    println!("{:?}", end.duration_since(start))
}

pub fn calculate_hash(input: String, times: usize) -> String {
    let mut hasher = Sha512_224::new();
    hasher.update(input);
    let output = hex::encode(hasher.finalize());
    match times {
        1 => output,
        _ => calculate_hash(output, times - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            calculate_hash("123asd".to_string(), 1),
            "071f28bd58c4ec6c154ea68fdc88949abf4734a5feaeacedfb0fd28d"
        );
    }
}
