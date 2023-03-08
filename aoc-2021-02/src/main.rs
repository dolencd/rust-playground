use std::{env, fs};

fn main() {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();

    let input_string_vec: Vec<&str> = input_string.split('\n').collect();

    let mut forward = 0;
    let mut depth = 0;
    let mut aim: i32 = 0;

    for input in input_string_vec {
        let mut iter = input.split(' ');
        let command = iter.next().unwrap();
        let amount: i32 = iter.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                forward += amount;
                depth += amount * aim;
            }
            "down" => {
                aim += amount;
                // depth+=amount;
            }
            "up" => {
                aim -= amount;
                // depth-=amount;
            }
            _ => (),
        }
    }
    println!("{}", forward * depth);
}
