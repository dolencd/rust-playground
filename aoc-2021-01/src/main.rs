use std::{env, fs};

fn main() {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();
    let input_lines_vec: Vec<&str> = input_string.split('\n').collect();

    let mut last: [i32; 3] = [-1, -1, -1];
    let mut last_sum: i32 = -1;
    let mut count = 0;

    for input in input_lines_vec {
        let input_num: i32 = input.parse().unwrap();
        last = [last[1], last[2], input_num];
        if last.into_iter().any(|v| v < 0) {
            continue;
        }

        let current_sum = last.into_iter().sum();

        if last_sum < 0 {
            last_sum = current_sum;
            continue;
        }

        if current_sum > last_sum {
            count += 1;
        }
        last_sum = current_sum;
    }
    println!("{}", count);
}
