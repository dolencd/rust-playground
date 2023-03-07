mod seafloor;
use seafloor::Seafloor;

use crate::seafloor::{Basin, Position};

fn main() {
    let seafloor: Seafloor = Default::default();
    let low_points: Vec<(Position, u8)> = seafloor
        .get_all_elements()
        .filter_map(|(pos, current_num)| {
            let is_lower_than_adjacent = seafloor
                .get_adjacent(&pos)
                .iter()
                .all(|adjacent_element| adjacent_element.1 > current_num.to_owned());
            if is_lower_than_adjacent {
                Some((pos, current_num.to_owned()))
            } else {
                None
            }
        })
        .collect();

    let mut deduped_basins: Vec<Basin> = low_points
        .iter()
        .map(|low_point| seafloor.get_basin_with_low_point(&low_point.0))
        .fold(vec![], |acc, current_basin| {
            if acc
                .iter()
                .all(|already_found_basin| *already_found_basin != current_basin)
            {
                return vec![current_basin]
                    .into_iter()
                    .chain(acc.into_iter())
                    .collect();
            }
            return acc;
        });

    deduped_basins.sort_by_cached_key(|basin| basin.len());
    deduped_basins.reverse();

    println!(
        "{}",
        deduped_basins[0].len() * deduped_basins[1].len() * deduped_basins[2].len()
    );
}
