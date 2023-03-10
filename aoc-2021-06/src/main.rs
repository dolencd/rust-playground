fn main() {
    // let mut input = vec![3, 4, 3, 1, 2];
    let input = vec![
        3, 5, 4, 1, 2, 1, 5, 5, 1, 1, 1, 1, 4, 1, 4, 5, 4, 5, 1, 3, 1, 1, 1, 4, 1, 1, 3, 1, 1, 5,
        3, 1, 1, 3, 1, 3, 1, 1, 1, 4, 1, 2, 5, 3, 1, 4, 2, 3, 1, 1, 2, 1, 1, 1, 4, 1, 1, 1, 1, 2,
        1, 1, 1, 3, 1, 1, 4, 1, 4, 1, 5, 1, 4, 2, 1, 1, 5, 4, 4, 4, 1, 4, 1, 1, 1, 1, 3, 1, 5, 1,
        4, 5, 3, 1, 4, 1, 5, 2, 2, 5, 1, 3, 2, 2, 5, 4, 2, 3, 4, 1, 2, 1, 1, 2, 1, 1, 5, 4, 1, 1,
        1, 1, 3, 1, 5, 4, 1, 5, 1, 1, 4, 3, 4, 3, 1, 5, 1, 1, 2, 1, 1, 5, 3, 1, 1, 1, 1, 1, 5, 1,
        1, 1, 1, 1, 1, 1, 2, 2, 5, 5, 1, 2, 1, 2, 1, 1, 5, 1, 3, 1, 5, 2, 1, 4, 1, 5, 3, 1, 1, 1,
        2, 1, 3, 1, 4, 4, 1, 1, 5, 1, 1, 4, 1, 4, 2, 3, 5, 2, 5, 1, 3, 1, 2, 1, 4, 1, 1, 1, 1, 2,
        1, 4, 1, 3, 4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 5, 1, 1, 1, 1, 2, 3, 1, 1, 2, 3, 1, 1, 3, 1, 1,
        3, 1, 3, 1, 3, 3, 1, 1, 2, 1, 3, 2, 3, 1, 1, 3, 5, 1, 1, 5, 5, 1, 2, 1, 2, 2, 1, 1, 1, 5,
        3, 1, 1, 3, 5, 1, 3, 1, 5, 3, 4, 2, 3, 2, 1, 3, 1, 1, 3, 4, 2, 1, 1, 3, 1, 1, 1, 1, 1, 1,
    ];

    let mut fish_per_age: [u128; 9] = [0; 9];

    for el in input {
        fish_per_age[el] += 1;
    }

    for day in 1..952 {
        let fish_at_0 = fish_per_age[0];
        fish_per_age[0] = fish_per_age[1];
        fish_per_age[1] = fish_per_age[2];
        fish_per_age[2] = fish_per_age[3];
        fish_per_age[3] = fish_per_age[4];
        fish_per_age[4] = fish_per_age[5];
        fish_per_age[5] = fish_per_age[6];
        fish_per_age[6] = fish_per_age[7];
        fish_per_age[7] = fish_per_age[8];

        fish_per_age[8] = fish_at_0;
        fish_per_age[6] += fish_at_0;
        println!("after {} days {}", day, fish_per_age.iter().sum::<u128>());
    }
}
