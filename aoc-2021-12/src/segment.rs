use std::fmt::Display;

use crate::cave::Cave;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub start: Cave,
    pub end: Cave,
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.start, self.end)
    }
}

impl Segment {
    pub fn new(input: &str) -> Option<Self> {
        let mut input_iter = input.split('-');
        let start: Cave = Cave::new(input_iter.next()?);
        let end: Cave = Cave::new(input_iter.next()?);
        Some(Segment { start, end })
    }

    pub fn get_reverse(&self) -> Self {
        Self {
            start: self.end.to_owned(),
            end: self.start.to_owned(),
        }
    }
}
