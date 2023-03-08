use std::{collections::HashMap, fmt::Display};

use crate::{cave::Cave, segment::Segment};

#[derive(Debug, Clone, Default)]
pub struct Path {
    segments: Vec<Segment>,
}

impl Path {
    pub fn can_add_segment(&self, new_segment: &Segment) -> bool {
        let Some(current_last_segment) = self.segments.last() else {
            return true;
        };

        if new_segment.end == Cave::new("start") {
            return false;
        }

        let number_of_small_caves_with_multiple_visits =
            self.number_of_small_caves_appearing_more_than_times(1);

        let number_of_existing_occurences = self
            .segments
            .iter()
            .filter(|existing_segment| {
                existing_segment.start.at_most_twice_travel
                    && existing_segment.start == new_segment.start
            })
            .count();

        if current_last_segment.end != new_segment.start
            || number_of_existing_occurences > 1
            || number_of_small_caves_with_multiple_visits > 1
        {
            return false;
        }

        true
    }
    pub fn add_segment(&mut self, segment: &Segment) -> Self {
        self.segments.push(segment.to_owned());
        self.to_owned()
    }

    fn number_of_small_caves_appearing_more_than_times(&self, times: usize) -> usize {
        self.count_segment_occurences()
            .iter()
            .filter(|(cave, count)| cave.at_most_twice_travel && **count > times)
            .count()
    }

    fn count_segment_occurences(&self) -> HashMap<Cave, usize> {
        let mut output_hashmap: HashMap<Cave, usize> = HashMap::new();
        for seg in &self.segments {
            let cave = seg.end.to_owned();
            match output_hashmap.get_mut(&cave) {
                Some(existing_count) => *existing_count += 1,
                None => {
                    output_hashmap.insert(cave, 1);
                }
            }
        }
        output_hashmap
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for seg in &self.segments {
            write!(f, "{},", seg)?;
        }
        Ok(())
    }
}
