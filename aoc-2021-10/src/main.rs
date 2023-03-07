use std::{fs, env};

use crate::row::{ChunkBoundary, Row};
use anyhow::Result;
mod row;

fn main() -> Result<()> {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();

    let mut scores: Vec<i128> = input_string
        .split("\n")
        .filter_map(|input_row| {
            let mut new_row: Row = Default::default();
            for char in input_row.chars() {
                let parsed_char: ChunkBoundary = char.try_into().map_or(None, |a| Some(a))?;
                if new_row.can_be_added(&parsed_char) {
                    new_row.add(&parsed_char);
                } else {
                    // println!(
                    //     "{:?},{:?},{:?}",
                    //     parsed_char,
                    //     new_row,
                    //     i32::from(parsed_char)
                    // );
                    return None;
                }
            }
            println!("{:?}", new_row.get_score_to_complete());
            Some(new_row.get_score_to_complete())
        })
        .collect();

    scores.sort();

    println!("{:?}", scores.get((scores.len() - 1) / 2));
    return Ok(());
}
