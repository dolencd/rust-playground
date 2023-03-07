use std::{
    fmt::{Debug, Display},
    ops::Add,
};

#[derive(Debug)]
pub struct Grid {
    positions: [[u8; 10]; 10],
}

#[derive(Clone, Copy)]
pub struct Position(pub i32, pub i32);

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.0, self.1)
    }
}

impl Add for Position {
    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
    type Output = Position;
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in self.positions {
            write!(f, "{:?}\n", r)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn get_pos(&self, pos: &Position) -> Option<u8> {
        Some(
            self.positions
                .get(pos.0 as usize)?
                .get(pos.1 as usize)?
                .to_owned(),
        )
    }

    pub fn get_adjacent<'a>(
        &'a self,
        pos: &'a Position,
    ) -> impl Iterator<Item = (Position, u8)> + 'a {
        return (-1..2).flat_map(move |x| {
            (-1..2).filter_map(move |y| {
                if x == 0 && y == 0 {
                    return None;
                }
                let output_pos = *pos + Position(x, y);
                if let Some(element) = self.get_pos(&output_pos) {
                    return Some((output_pos, element));
                } else {
                    return None;
                }
            })
        });
    }

    pub fn flash(&mut self) -> i32 {
        let tmp = self.get_all_elements().collect::<Vec<(Position, u8)>>();
        let mut flashes = 0;
        for (pos, _) in tmp {
            flashes += self.flash_rec(&pos);
        }

        for row_array in self.positions.iter_mut() {
            for element in row_array.iter_mut() {
                if *element > 9 {
                    *element = 0
                };
            }
        }

        return flashes;
    }

    pub fn flash_rec(&mut self, pos: &Position) -> i32 {
        let me: &mut u8 = self
            .positions
            .get_mut(pos.1 as usize)
            .unwrap()
            .get_mut(pos.0 as usize)
            .unwrap();
        *me += 1;
        // println!("{:?},{}", pos, me);
        if *me == 10 {
            // println!("{:?} flashed with {:?}", pos, me);
            let adjacent_positions: Vec<(Position, u8)> = self.get_adjacent(pos).collect();
            let mut flashes = 1;
            for (adjacent_position, _) in adjacent_positions {
                flashes += self.flash_rec(&adjacent_position);
            }
            return flashes;
        }

        return 0;
    }

    pub fn get_all_elements(&self) -> impl Iterator<Item = (Position, u8)> + '_ {
        self.positions
            .iter()
            .enumerate()
            .map(|(current_row, row_array)| {
                row_array
                    .iter()
                    .enumerate()
                    .map(move |(current_column, row_element)| {
                        (
                            Position(
                                current_row.try_into().unwrap(),
                                current_column.try_into().unwrap(),
                            ),
                            row_element.to_owned(),
                        )
                    })
            })
            .flatten()
    }

    fn set_value(&mut self, position: &Position, value: u8) -> bool {
        let Some(row) = self.positions.get_mut(position.1 as usize) else {
                return false;};

        let Some(_) = row.get(position.0 as usize) else {
                    return false;};

        row[position.0 as usize] = value;
        return true;
    }

    fn finish_flash(&mut self) -> () {}
}

// impl Default for Grid {
//     fn default() -> Self {
//         Grid {
//             positions: [
//                 [1, 1, 1, 1, 1],
//                 [1, 9, 9, 9, 1],
//                 [1, 9, 1, 9, 1],
//                 [1, 9, 9, 9, 1],
//                 [1, 1, 1, 1, 1],
//             ],
//         }
//     }
// }

impl Default for Grid {
    fn default() -> Self {
        Grid {
            positions: [
                [4, 7, 4, 3, 3, 7, 8, 3, 1, 8],
                [4, 6, 6, 4, 2, 1, 2, 8, 4, 4],
                [2, 5, 3, 5, 6, 6, 7, 8, 8, 4],
                [3, 2, 7, 3, 3, 6, 3, 8, 6, 1],
                [2, 2, 8, 2, 4, 3, 2, 6, 1, 2],
                [2, 1, 6, 6, 6, 1, 2, 1, 3, 4],
                [3, 7, 7, 6, 3, 3, 4, 5, 1, 3],
                [8, 1, 2, 3, 8, 5, 2, 5, 8, 3],
                [8, 1, 8, 1, 7, 8, 6, 6, 8, 5],
                [4, 3, 6, 2, 5, 3, 3, 1, 7, 4],
            ],
        }
    }
}
