#![feature(anonymous_lifetime_in_impl_trait)]
mod seafloor;

use seafloor::{point_from_comma_separated_pair, Point, Seafloor};
use std::{fs, env};

use crate::seafloor::{is_line_diagonal, is_line_straight};

fn main() {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();

    let hotsprings = input_string.split("\n");

    let mut seafloor: Seafloor = Default::default();

    let parsed_straight_hotsprings = hotsprings
        .map(|hotspring_str| -> (Point, Point) {
            let mut hotspring_iter = hotspring_str.split_whitespace();
            let point_1_str = hotspring_iter.next().unwrap();
            hotspring_iter.next(); // arrow
            let point_2_str = hotspring_iter.next().unwrap();
            let tmp = (
                point_from_comma_separated_pair(point_1_str),
                point_from_comma_separated_pair(point_2_str),
            );
            // println!("{:?}, {:?}", tmp.0,tmp.1);
            return tmp;
        })
        .filter(|(point_1, point_2)| {
            is_line_straight(point_1, point_2) || is_line_diagonal(point_1, point_2)
        });

    for (start, end) in parsed_straight_hotsprings {
        if is_line_diagonal(&start, &end) {
            seafloor.add_diag(&start, &end);
        } else {
            seafloor.add_rect(&start, &end)
        };
    }
    // print!("{}", seafloor);
    print!("{}", seafloor.count_squares_greater_than(1));
    // println!("Done, {:?}", parsed_straight_hotsprings);
    return ();
}
