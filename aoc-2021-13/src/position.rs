use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn new(input_string: &str) -> Option<Self> {
        let mut input = input_string.split(',');
        Some(Position(
            input.next()?.parse().ok()?,
            input.next()?.parse().ok()?,
        ))
    }

    pub fn move_position(&mut self, x: i32, y: i32) {
        self.0 += x;
        self.1 += y;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}
