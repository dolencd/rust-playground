use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cave {
    pub name: String,
    pub at_most_twice_travel: bool,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Cave {
    pub fn new(value: &str) -> Self {
        Cave {
            name: value.to_string(),
            at_most_twice_travel: value.to_lowercase() == value,
        }
    }
}
