#![feature(anonymous_lifetime_in_impl_trait)]
mod board;

use board::Board;
use std::{fs, env};

fn main() {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();

    let mut sections = input_string.split("\n\n");

    let draws = sections
        .next()
        .unwrap()
        .split(",")
        .map(|num_str| num_str.parse::<u32>().unwrap());

    let mut boards: Vec<Board> = sections
        .map(|section| {
            section
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|num_vec| Board::from(num_vec))
        .collect();

    for draw in draws {
        for i in 0..boards.len() {
            boards[i].process_number(draw);
            if boards
                .iter()
                .filter(|board| !board.is_board_solved())
                .count()
                == 0
            {
                println!("{}", boards[i].sum_of_unmarked_numbers() * draw);
                return;
            }
        }
    }
    println!("no results");
    return;
}

// fn first_challenge(draws: Vec<u32>, remaining_sections: impl Iterator<Item = &str>) -> Option<u32> {

//     let mut boards: Vec<Board> = remaining_sections
//         .map(|section| {
//             section
//                 .split_whitespace()
//                 .map(|num_str| num_str.parse::<u32>().unwrap())
//                 .collect::<Vec<u32>>()
//         })
//         .map(|num_vec| Board::from(num_vec))
//         .collect();

//     for draw in draws {
//         for board in &mut boards {
//             board.process_number(draw);
//             if board.is_board_solved() {
//                 return Some(board.sum_of_unmarked_numbers() * draw);
//             }
//         }
//     }
//     return None;
// }
