use std::fmt::Display;

use crate::position::Position;

pub struct Grid(Vec<Position>);

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if self
                    .0
                    .iter()
                    .any(|pos_internal| *pos_internal == Position(x, y))
                {
                    write!(f, "{}", '#')?;
                } else {
                    write!(f, "{}", '.')?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(input: &str) -> Self {
        Self(
            input
                .split("\n")
                .map(|s| Position::new(s).unwrap())
                .collect(),
        )
    }

    pub fn get_width(&self) -> i32 {
        self.0
            .iter()
            .max_by_key(|pos| pos.0)
            .unwrap_or(&Position(0, 0))
            .0
            + 1
    }

    pub fn get_height(&self) -> i32 {
        self.0
            .iter()
            .max_by_key(|pos| pos.1)
            .unwrap_or(&Position(0, 0))
            .1
            + 1
    }

    fn clear_duplicates(&mut self) {
        self.0 = self
            .0
            .iter_mut()
            .fold(vec![], |mut acc: Vec<Position>, pos: &mut Position| {
                if !acc.iter().any(|pos_internal| pos_internal == pos) {
                    acc.push(pos.to_owned());
                }
                return acc;
            })
    }

    pub fn fold_x(&mut self, column: i32) {
        self.0.iter_mut().for_each(|pos| {
            if pos.0 > column {
                pos.move_position(-2 * (pos.0 - column), 0);
            }
        });
        self.clear_duplicates();
    }

    pub fn fold_y(&mut self, column: i32) {
        self.0.iter_mut().for_each(|pos| {
            if pos.1 > column {
                pos.move_position(0, -2 * (pos.1 - column));
            }
        });
        self.clear_duplicates();
    }

    pub fn number_of_dots(&self) -> usize {
        self.0.len()
    }
}
