pub struct Board {
    places: [(u32, bool); 25],
}

fn is_solved(mut row_or_column: impl Iterator<Item = (u32, bool)>) -> bool {
    row_or_column.all(|el| el.1)
}

impl From<Vec<u32>> for Board {
    fn from(input: Vec<u32>) -> Board {
        let arr: [u32; 25] = input.try_into().unwrap();
        Board {
            places: arr.map(|num| (num, false)),
        }
    }
}

impl<'a> Board {
    pub fn is_board_solved(&self) -> bool {
        for i in 0..5 {
            if is_solved(self.get_column(i)) || is_solved(self.get_row(i)) {
                return true;
            }
        }
        false
    }

    pub fn sum_of_unmarked_numbers(&self) -> u32 {
        self.places
            .iter()
            .filter(|(_, is_marked)| !is_marked)
            .map(|(number, _)| number)
            .sum()
    }

    fn get_column(&'a self, row_index: usize) -> impl Iterator<Item = (u32, bool)> + '_ {
        self.places
            .iter()
            .enumerate()
            .filter(move |el| (el.0 % 5) == row_index)
            .map(|el| *el.1)
    }

    fn get_row(&'a self, row_index: usize) -> impl Iterator<Item = (u32, bool)> + '_ {
        self.places[row_index * 5..(row_index + 1) * 5]
            .iter()
            .map(|v| v.to_owned())
    }

    pub fn process_number(&mut self, number_to_process: u32) -> Option<()> {
        let index = self
            .places
            .iter()
            .position(|place| place.0 == number_to_process)?;

        self.places[index].1 = true;
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn play_a_game() {
        let mut board: Board = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24,
        ]
        .into();
        assert_eq!(
            board.places,
            [
                (0, false),
                (1, false),
                (2, false),
                (3, false),
                (4, false),
                (5, false),
                (6, false),
                (7, false),
                (8, false),
                (9, false),
                (10, false),
                (11, false),
                (12, false),
                (13, false),
                (14, false),
                (15, false),
                (16, false),
                (17, false),
                (18, false),
                (19, false),
                (20, false),
                (21, false),
                (22, false),
                (23, false),
                (24, false),
            ]
        );
        board.process_number(0);
        assert_eq!(board.places[0], (0, true));
        // board.process_number(12);
        // assert!(board.places[12].1);
    }
}
