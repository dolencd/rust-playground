#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChunkBoundary {
    LA,
    RA,
    LB,
    RB,
    LC,
    RC,
    LD,
    RD,
}

impl From<ChunkBoundary> for i32 {
    fn from(value: ChunkBoundary) -> Self {
        match value {
            ChunkBoundary::LA | ChunkBoundary::LB | ChunkBoundary::LC | ChunkBoundary::LD => 0,
            ChunkBoundary::RA => 1,
            ChunkBoundary::RB => 2,
            ChunkBoundary::RC => 3,
            ChunkBoundary::RD => 4,
        }
    }
}

impl TryFrom<char> for ChunkBoundary {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '(' => ChunkBoundary::LA,
            ')' => ChunkBoundary::RA,
            '[' => ChunkBoundary::LB,
            ']' => ChunkBoundary::RB,
            '{' => ChunkBoundary::LC,
            '}' => ChunkBoundary::RC,
            '<' => ChunkBoundary::LD,
            '>' => ChunkBoundary::RD,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub struct Row {
    chars: Vec<ChunkBoundary>,
}

impl TryFrom<Vec<char>> for Row {
    type Error = ();
    fn try_from(values: Vec<char>) -> Result<Self, Self::Error> {
        let mut row: Row = Default::default();
        for value in values {
            let parsed_char: ChunkBoundary = value.try_into()?;
            if !row.can_be_added(&parsed_char) {
                return Err(());
            }
            row.add(&parsed_char);
        }
        return Ok(row);
    }
}

impl Row {
    pub fn can_be_added(&self, chunk_boundary_to_add: &ChunkBoundary) -> bool {
        let Some(latest_chunk_boundary ) = self.chars.last() else {
            return true;
        };

        match chunk_boundary_to_add {
            ChunkBoundary::RA => *latest_chunk_boundary == ChunkBoundary::LA,
            ChunkBoundary::RB => *latest_chunk_boundary == ChunkBoundary::LB,
            ChunkBoundary::RC => *latest_chunk_boundary == ChunkBoundary::LC,
            ChunkBoundary::RD => *latest_chunk_boundary == ChunkBoundary::LD,
            _ => true,
        }
    }

    pub fn add(&mut self, chunk_boundary_to_add: &ChunkBoundary) -> () {
        match chunk_boundary_to_add {
            ChunkBoundary::RA | ChunkBoundary::RB | ChunkBoundary::RC | ChunkBoundary::RD => {
                self.chars.pop();
                ()
            }
            ChunkBoundary::LA | ChunkBoundary::LB | ChunkBoundary::LC | ChunkBoundary::LD => {
                self.chars.push(*chunk_boundary_to_add)
            }
        }
    }

    pub fn get_score_to_complete(&self) -> i128 {
        let mut closing_chars: Vec<ChunkBoundary> = self
            .chars
            .iter()
            .map(|c| match *c {
                ChunkBoundary::LA => ChunkBoundary::RA,
                ChunkBoundary::LB => ChunkBoundary::RB,
                ChunkBoundary::LC => ChunkBoundary::RC,
                ChunkBoundary::LD => ChunkBoundary::RD,
                _ => unreachable!(),
            })
            .collect();
        closing_chars.reverse();
        let mut total: i128 = 0;
        for char in closing_chars {
            total *= 5;
            total += i32::from(char) as i128;
        }
        return total;
    }
}

impl Default for Row {
    fn default() -> Self {
        Row { chars: vec![] }
    }
}
