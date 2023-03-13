use crate::world::{World, WORLD_SIZE};

impl World {
    pub fn new() -> Self {
        let world_bytes: [[char; WORLD_SIZE as usize]; WORLD_SIZE as usize] =
            include_str!("./map.txt")
                .split('\n')
                .rev()
                .map(|map_row| {
                    let row_arr: [char; WORLD_SIZE as usize] =
                        map_row.chars().collect::<Vec<_>>().try_into().unwrap();
                    row_arr
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
        Self {
            squares: world_bytes,
        }
    }
}
