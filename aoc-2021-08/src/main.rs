use itertools::Itertools;
use std::{env, fs, vec};

// const VALID_GROUPS: [Vec<char>; 10] = [
//     vec!['a', 'b', 'c', 'e', 'f', 'g'],      // 0
//     vec!['c', 'f'],                          // 1
//     vec!['a', 'c', 'd', 'f', 'g'],           // 2
//     vec!['a', 'c', 'd', 'f', 'g'],           // 3
//     vec!['b', 'c', 'd', 'f'],                // 4
//     vec!['a', 'b', 'd', 'f', 'g'],           // 5
//     vec!['a', 'b', 'd', 'e', 'f', 'g'],      // 6
//     vec!['a', 'c', 'f'],                     // 7
//     vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'], // 8
//     vec!['a', 'b', 'c', 'd', 'f', 'g'],      // 9
// ];

fn get_simple_mapping() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5, 6]
}

fn get_valid_group_num() -> [Vec<u32>; 10] {
    [
        vec![0, 1, 2, 4, 5, 6],    // 0
        vec![2, 5],                // 1
        vec![0, 2, 3, 5, 6],       // 2
        vec![0, 2, 3, 5, 6],       // 3
        vec![1, 2, 3, 5],          // 4
        vec![0, 1, 3, 5, 6],       // 5
        vec![0, 1, 3, 4, 5, 6],    // 6
        vec![0, 2, 5],             // 7
        vec![0, 1, 2, 3, 4, 5, 6], // 8
        vec![0, 1, 2, 3, 5, 6],    // 9
    ]
}

fn main() {
    let input_string = fs::read_to_string(
        env::current_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join("input.txt"),
    )
    .unwrap();

    let display_strings = input_string.split('\n');

    let sum_of_digits: u32 = display_strings
        .map(|display_string| -> u32 {
            let (all_samples, all_outputs) = display_string.split_once("").unwrap();
            let found_mapping = get_valid_mapping(all_samples);

            all_outputs
                .split_whitespace()
                .map(|num_str| get_number_using_mapping(&found_mapping, &str_to_code(num_str)))
                .sum()
        })
        .sum();

    print!("{:?}", sum_of_digits);
    // print!("{}", seafloor.count_squares_greater_than(1));
    // println!("Done, {:?}", parsed_straight_hotsprings);
}

fn get_number_using_mapping(mapping: &Vec<u32>, code: &Vec<u32>) -> u32 {
    let mapped_segments: Vec<u32> = code
        .iter()
        .map(|unmapped_segment| mapping[unmapped_segment.to_owned() as usize])
        .sorted()
        .collect();

    get_valid_group_num()
        .iter()
        .enumerate()
        .find(|(_, valid_group)| vectors_are_same(valid_group, &mapped_segments))
        .unwrap()
        .0 as u32
}

fn get_valid_mapping(inputs: &str) -> Vec<u32> {
    // let all_possible_mappings = all_possible_mappings_input.to_owned();
    let found_words: Vec<Vec<u32>> = inputs
        .split_whitespace()
        .map(|word| {
            word.chars()
                .sorted()
                .map(|char| letter_to_code(&char))
                .collect()
        })
        .collect();

    get_simple_mapping()
        .into_iter()
        .permutations(get_simple_mapping().len())
        .unique()
        .find(|mapping_to_test| {
            found_words
                .iter()
                .all(|word| is_valid_mapping(mapping_to_test, word))
        })
        .unwrap()
}

fn is_valid_mapping(mapping: &Vec<u32>, input: &Vec<u32>) -> bool {
    let decoded_input: Vec<u32> = input
        .iter()
        .map(|input_num| mapping[*input_num as usize])
        .sorted()
        .collect();
    get_valid_group_num()
        .iter()
        .any(|valid_group| vectors_are_same(valid_group, &decoded_input))
}

fn str_to_code(input: &str) -> Vec<u32> {
    input.chars().map(|c| letter_to_code(&c)).collect()
}

fn vectors_are_same<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    a.iter().zip_eq(b.iter()).all_equal()
}

fn letter_to_code(input: &char) -> u32 {
    // println!("{}", input);
    let parsed_digit = input.to_digit(20).unwrap();
    parsed_digit - 10
}

#[cfg(test)]
mod tests {
    use crate::letter_to_code;

    #[test]
    fn test_letter_to_code() {
        assert_eq!(letter_to_code(&'a'), 0);
        assert_eq!(letter_to_code(&'b'), 1);
        assert_eq!(letter_to_code(&'c'), 2);
        assert_eq!(letter_to_code(&'d'), 3);
        assert_eq!(letter_to_code(&'e'), 4);
        assert_eq!(letter_to_code(&'f'), 5);
        assert_eq!(letter_to_code(&'g'), 6);
    }
}

// 0 - 6
// 1 - 2
// 2 - 5
// 3 - 5
// 4 - 4
// 5 - 5
// 6 - 6
// 7 - 3
// 8 - 7
// 9 - 6
