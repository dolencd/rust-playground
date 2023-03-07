use std::fmt::{Debug, Display};

const SEAFLOOR_SIZE: usize = 1000;

pub struct Seafloor {
    positions: [[i32; SEAFLOOR_SIZE]; SEAFLOOR_SIZE],
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub fn point_from_comma_separated_pair(input_str: &str) -> Point {
    let mut input_str_split = input_str.split(",");
    return Point {
        x: input_str_split.next().unwrap().parse().unwrap(),
        y: input_str_split.next().unwrap().parse().unwrap(),
    };
}

pub fn is_line_straight(a: &Point, b: &Point) -> bool {
    a.x == b.x || a.y == b.y
}

pub fn is_line_diagonal(a: &Point, b: &Point) -> bool {
    i32::abs(a.x - b.x) == i32::abs(a.y - b.y)
}

impl Default for Seafloor {
    fn default() -> Self {
        return Seafloor {
            positions: [[0; SEAFLOOR_SIZE]; SEAFLOOR_SIZE],
        };
    }
}

impl Display for Seafloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..10 {
            for column in 0..10 {
                f.write_str(&self.positions[column][row].to_string())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Debug for Seafloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..10 {
            for column in 0..10 {
                f.write_str(&self.positions[column][row].to_string())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl PartialEq for Seafloor {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions
    }
}

impl<'a> Seafloor {
    pub fn count_squares_greater_than(&self, threshold: i32) -> usize {
        self.positions
            .iter()
            .flat_map(|column| column)
            .filter(|num| num > &&threshold)
            .count()
    }

    pub fn add_rect(&mut self, a: &Point, b: &Point) -> () {
        let (start_x, end_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (start_y, end_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

        for x in start_x..end_x + 1 {
            for y in start_y..end_y + 1 {
                self.positions[x as usize][y as usize] += 1;
            }
        }
    }

    pub fn add_diag(&mut self, a: &Point, b: &Point) -> () {
        let dir_x = i32::clamp(b.x - a.x, -1, 1);
        let dir_y = i32::clamp(b.y - a.y, -1, 1);
        let dist = i32::abs(a.x - b.x);

        for step in 0..dist + 1 {
            self.positions[(a.x + dir_x * step) as usize][(a.y + dir_y * step) as usize] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::seafloor::Point;

    use super::{is_line_straight, Seafloor};

    #[test]
    fn it_works() {
        let mut seafloor: Seafloor = Default::default();
        // assert_eq!(seafloor.positions.len(), 1000);
        assert_eq!(seafloor.count_squares_greater_than(0), 0);
        seafloor.add_rect(&Point { x: 1, y: 2 }, &Point { x: 2, y: 2 });
        assert_eq!(seafloor.count_squares_greater_than(0), 2);
        seafloor.add_rect(&Point { x: 1, y: 2 }, &Point { x: 3, y: 2 });
        assert_eq!(seafloor.count_squares_greater_than(0), 3);
        assert_eq!(seafloor.count_squares_greater_than(1), 2);
    }

    #[test]
    fn straight_line() {
        assert!(is_line_straight(
            &Point { x: 1, y: 2 },
            &Point { x: 2, y: 2 }
        ));
        assert!(is_line_straight(
            &Point { x: 2, y: 2 },
            &Point { x: 2, y: 2 }
        ));
        assert!(is_line_straight(
            &Point { x: 3, y: 4 },
            &Point { x: 1, y: 4 }
        ));

        // not straight
        assert!(!is_line_straight(
            &Point { x: 1, y: 2 },
            &Point { x: 2, y: 1 }
        ));
        assert!(!is_line_straight(
            &Point { x: 1, y: 2 },
            &Point { x: 3, y: 4 }
        ));
    }

    #[test]
    fn filling_1() {
        let mut seafloor_a: Seafloor = Default::default();
        let mut seafloor_b: Seafloor = Default::default();
        seafloor_a.add_rect(&Point { x: 2, y: 3 }, &Point { x: 5, y: 6 });
        seafloor_b.add_rect(&Point { x: 5, y: 6 }, &Point { x: 2, y: 3 });
        assert_eq!(seafloor_a, seafloor_b)
    }

    #[test]
    fn filling_2() {
        let mut seafloor_a: Seafloor = Default::default();
        let mut seafloor_b: Seafloor = Default::default();
        seafloor_a.add_rect(&Point { x: 2, y: 6 }, &Point { x: 5, y: 3 });
        seafloor_b.add_rect(&Point { x: 5, y: 3 }, &Point { x: 2, y: 6 });
        assert_eq!(seafloor_a, seafloor_b)
    }
}
