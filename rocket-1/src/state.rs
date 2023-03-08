pub mod state {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};
    use strum_macros::Display;
    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct State {
        pub num: i32,
        pub arr: Vec<i32>,
    }

    impl Display for State {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Current num: {}, current arr: {:?}", self.num, self.arr)
        }
    }

    impl PartialEq for State {
        fn eq(&self, other: &Self) -> bool {
            self.num == other.num
        }
    }

    pub fn save_state(name: &str, state: &State) -> Result<(), std::io::Error> {
        std::fs::write(name, serde_json::to_string_pretty(&state)?)
    }

    #[derive(Debug, Display)]
    pub enum ReadStateError {
        IO(std::io::Error),
        UTF8(std::str::Utf8Error),
        JSON(serde_json::Error),
    }

    impl From<std::io::Error> for ReadStateError {
        fn from(err: std::io::Error) -> Self {
            ReadStateError::IO(err)
        }
    }

    impl From<std::str::Utf8Error> for ReadStateError {
        fn from(err: std::str::Utf8Error) -> Self {
            ReadStateError::UTF8(err)
        }
    }

    impl From<serde_json::Error> for ReadStateError {
        fn from(err: serde_json::Error) -> Self {
            ReadStateError::JSON(err)
        }
    }

    pub fn read_state(name: &str) -> Result<State, ReadStateError> {
        let read_data = std::fs::read(name)?;
        let str_to_parse = std::str::from_utf8(&read_data)?;
        let state: State = serde_json::from_str(str_to_parse)?;
        Ok(state)
    }

    pub fn delete_state(name: &str) -> Result<(), std::io::Error> {
        std::fs::remove_file(name)
    }

    pub fn exists(name: &str) -> bool {
        std::path::Path::new(name).exists()
    }
}

#[cfg(test)]
mod tests {
    use crate::state::state::delete_state;

    use super::state::{self, read_state, State};

    #[test]
    fn read_empty_state() {
        let res = read_state("test");
        assert!(res.is_err())
    }

    #[test]
    fn write_and_read_state() {
        let state_1 = State {
            num: 12,
            arr: vec![1, 2],
        };

        let state_2 = State {
            num: 23,
            arr: vec![4, 5],
        };
        assert!(!state::exists("state1"));
        state::save_state("state_1", &state_1);
        state::save_state("state_2", &state_2);
        let saved_state_1 = state::read_state("state_1").unwrap();
        let saved_state_2 = state::read_state("state_2").unwrap();

        assert!(state::exists("state_1"));

        assert_eq!(state_1, saved_state_1);
        assert_eq!(state_2, saved_state_2);

        delete_state("state_1");
        delete_state("state_2");

        assert!(!state::exists("state1"));
        assert!(state::read_state("state_1").is_err());
        assert!(state::read_state("state_2").is_err());
    }
}
