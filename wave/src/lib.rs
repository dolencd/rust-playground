pub fn wave(input: &str) -> Vec<String> {
    let lowercase_input = input.to_lowercase();
    input
        .to_lowercase()
        .chars()
        .enumerate()
        .filter_map(|(index, char_to_test)| {
            if !char_to_test.is_alphabetic() {
                return None;
            }

            let mut owned_input = lowercase_input.to_owned();

            owned_input.replace_range(
                index..(index + 1),
                char_to_test.to_uppercase().collect::<String>().as_str(),
            );

            return Some(owned_input);
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(wave(""), Vec::<&str>::new());

        assert_eq!(wave("a"), vec!["A"]);

        assert_eq!(wave("ab"), vec!["Ab", "aB"]);

        assert_eq!(wave("aB"), vec!["Ab", "aB"]);

        assert_eq!(wave("AB"), vec!["Ab", "aB"]);

        assert_eq!(wave("A.B"), vec!["A.b", "a.B"]);

        assert_eq!(
            wave("A.,<>!@#$%^&*()_+-={]:;'\"B"),
            vec!["A.,<>!@#$%^&*()_+-={]:;'\"b", "a.,<>!@#$%^&*()_+-={]:;'\"B"]
        );

        assert_eq!(wave("A b"), vec!["A b", "a B"]);

        assert_eq!(wave("A\nb\tc"), vec!["A\nb\tc", "a\nB\tc", "a\nb\tC"]);

        assert_eq!(wave("MARY"), vec!["Mary", "mAry", "maRy", "marY"]);

        assert_eq!(wave("maRy"), vec!["Mary", "mAry", "maRy", "marY"]);

        assert_eq!(
            wave("Hello there!"),
            vec![
                "Hello there!",
                "hEllo there!",
                "heLlo there!",
                "helLo there!",
                "hellO there!",
                "hello There!",
                "hello tHere!",
                "hello thEre!",
                "hello theRe!",
                "hello therE!",
            ]
        );
    }
}
