use std::{env, fs};

use anyhow::Result;
use grid::Grid;
mod grid;
mod position;

fn main() -> Result<()> {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();
    let mut input_string_iter = input_string.split("\n\n");
    let mut grid = Grid::new(input_string_iter.next().unwrap());

    // println!("{}", grid);

    let folds = extract_fold_strings(input_string_iter.next().unwrap());

    // println!("{:?}", folds);

    for fold in folds {
        match fold {
            ('x', column) => grid.fold_x(column),
            ('y', row) => grid.fold_y(row),
            (_, _) => unreachable!(),
        }
        // println!("{}\n", grid);
    }

    println!("{}", grid);

    print!("dots: {}", grid.number_of_dots());

    Ok(())
}

fn extract_fold_strings(input: &str) -> Vec<(char, i32)> {
    input
        .split('\n')
        .map(|row| {
            (
                row.chars().nth(11).unwrap(),
                row.split_at(13).1.parse().unwrap(),
            )
        })
        .collect()
}
