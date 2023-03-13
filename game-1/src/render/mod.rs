use crate::world::{Position, VIEW_SIZE};

pub trait Renderable {
    fn get_at_position(&self, position: &Position) -> Option<char>;
}

pub fn render(layers: &[&dyn Renderable], center: &Position) -> String {
    ((center.1 - VIEW_SIZE - 1)..(center.1 + VIEW_SIZE))
        .rev()
        .map(|y| {
            ((center.0 - VIEW_SIZE - 1)..(center.0 + VIEW_SIZE))
                .map(|x| {
                    match layers
                        .iter()
                        .find_map(|renderable| renderable.get_at_position(&Position(x, y)))
                    {
                        None => 'x',
                        Some(c) => c,
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
