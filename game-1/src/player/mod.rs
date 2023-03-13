use crate::{render::Renderable, world::Position};

#[derive(Debug, Clone, Copy, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

pub struct Player {
    pub position: Position,
    pub direction: Direction,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Position(2, 2),
            direction: Default::default(),
        }
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Down => self.position.1 -= 1,
            Direction::Up => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }
}

impl Renderable for Player {
    fn get_at_position(&self, position: &Position) -> Option<char> {
        if self.position == *position {
            return Some('*');
        }
        None
    }
}
