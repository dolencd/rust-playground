use std::{env, fs};

fn main() {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();

    let rows: Vec<&str> = input_string.split('\n').collect();

    let rows_digits: Vec<[i32; 12]> = rows.into_iter().map(row_to_int_vec).collect();

    println!(
        "{}",
        to_i32(&filter_by_most_common_digit(&rows_digits))
            * to_i32(&filter_by_least_common_digit(&rows_digits))
    );
}

fn filter_by_most_common_digit(input_arr: &Vec<[i32; 12]>) -> [i32; 12] {
    let mut array_to_filter = input_arr.to_owned();
    for i in 0..12 {
        if array_to_filter.len() == 1 {
            continue;
        }
        let (count_0, count_1) = count_0_and_1(&array_to_filter, i);
        assert_eq!(count_0 + count_1, array_to_filter.len());
        let most_common_digit = if count_1 >= count_0 { 1 } else { 0 };
        array_to_filter = filter_array_based_on_digit(&array_to_filter, i, most_common_digit)
    }
    array_to_filter[0]
}

fn filter_by_least_common_digit(input_arr: &Vec<[i32; 12]>) -> [i32; 12] {
    let mut array_to_filter = input_arr.to_owned();
    for i in 0..12 {
        if array_to_filter.len() == 1 {
            continue;
        }
        let (count_0, count_1) = count_0_and_1(&array_to_filter, i);
        assert_eq!(count_0 + count_1, array_to_filter.len());
        let least_common_digit = if count_1 < count_0 { 1 } else { 0 };
        array_to_filter = filter_array_based_on_digit(&array_to_filter, i, least_common_digit)
    }
    array_to_filter[0]
}

fn count_0_and_1(input_arr: &Vec<[i32; 12]>, index: usize) -> (usize, usize) {
    let digits: Vec<i32> = input_arr.iter().map(|digit_arr| digit_arr[index]).collect();
    let count_1 = digits
        .to_owned()
        .into_iter()
        .filter(|digit| *digit == 1)
        .count();
    let count_0 = digits.into_iter().filter(|digit| *digit == 0).count();
    (count_0, count_1)
}

fn filter_array_based_on_digit(
    input_arr: &Vec<[i32; 12]>,
    index_to_filter: usize,
    required_digit: i32,
) -> Vec<[i32; 12]> {
    input_arr
        .to_owned()
        .into_iter()
        .filter(|digit_arr| digit_arr[index_to_filter] == required_digit)
        .collect()
}

fn do_task_1(rows_digits: Vec<[i32; 12]>) {
    let mut gamma_rate: [i32; 12] = [0; 12];
    let mut epsilon_rate: [i32; 12] = [0; 12];

    for i in 0..12 {
        let number_of_ones: i32 = rows_digits
            .iter()
            .map(|row_digit_arr| row_digit_arr[i])
            .sum();
        let length = rows_digits.len() / 2;
        if number_of_ones as usize > length {
            gamma_rate[i] = 1;
            epsilon_rate[i] = 0;
        } else {
            gamma_rate[i] = 0;
            epsilon_rate[i] = 1;
        }
    }

    println!("{}", to_i32(&gamma_rate) * to_i32(&epsilon_rate))
}

fn to_i32(slice: &[i32]) -> i32 {
    slice
        .iter()
        .rev()
        .fold((0, 1), |(acc, mul), &bit| {
            (acc + (mul * (1 & bit)), mul.wrapping_add(mul))
        })
        .0
}

fn row_to_int_vec(row_str: &str) -> [i32; 12] {
    // let _row_str_split: Vec<&str> = row_str.split(' ').collect();
    let digits_vec: Vec<i32> = row_str
        .split(' ')
        .map(|digit_str| digit_str.parse().unwrap())
        .collect();
    // println!("{:?}", digits_vec);
    [
        digits_vec[0],
        digits_vec[1],
        digits_vec[2],
        digits_vec[3],
        digits_vec[4],
        digits_vec[5],
        digits_vec[6],
        digits_vec[7],
        digits_vec[8],
        digits_vec[9],
        digits_vec[10],
        digits_vec[11],
    ]
}
