mod game_map;

use crate::{position::Position, render::Renderable};

pub const WORLD_SIZE: i32 = 64;
pub const VIEW_SIZE: i32 = 10;

pub struct World {
    pub(crate) squares: [[char; WORLD_SIZE as usize]; WORLD_SIZE as usize],
}

impl World {
    pub fn is_solid(&self, pos: &Position) -> bool {
        match self.get_square_contents(pos) {
            None => true,
            Some(' ') => false,
            _ => true,
        }
    }

    pub fn get_square_contents_mut(&mut self, pos: &Position) -> Option<&mut char> {
        Some(
            self.squares
                .get_mut(pos.1 as usize)?
                .get_mut(pos.0 as usize)?,
        )
    }

    pub fn get_square_contents(&self, pos: &Position) -> Option<&char> {
        Some(self.squares.get(pos.1 as usize)?.get(pos.0 as usize)?)
    }
}

impl Renderable for World {
    fn get_at_position(&self, position: &Position) -> Option<&char> {
        self.get_square_contents(position)
    }
}
