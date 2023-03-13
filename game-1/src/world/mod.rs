mod game_map;
pub mod position;

pub use position::Position;

use crate::render::Renderable;

pub const WORLD_SIZE: i32 = 64;
pub const VIEW_SIZE: i32 = 12;

pub struct World {
    pub(crate) squares: [[char; WORLD_SIZE as usize]; WORLD_SIZE as usize],
}

impl World {
    pub fn render(&self, center: &Position) -> String {
        ((center.1 - VIEW_SIZE - 1)..(center.1 + VIEW_SIZE))
            .rev()
            .map(|y| {
                ((center.0 - VIEW_SIZE - 1)..(center.0 + VIEW_SIZE))
                    .map(|x| match self.get_square_contents(&Position(x, y)) {
                        None => 'x',
                        Some(c) => c,
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_square_contents(&self, pos: &Position) -> Option<char> {
        Some(*self.squares.get(pos.1 as usize)?.get(pos.0 as usize)?)
    }
}

impl Renderable for World {
    fn get_at_position(&self, position: &Position) -> Option<char> {
        self.get_square_contents(position)
    }
}
