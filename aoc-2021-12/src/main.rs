use anyhow::Result;
use cave::Cave;
use path::Path;

use crate::segment::Segment;
mod cave;
mod path;
mod segment;

struct WrappedVec<T>(Vec<T>);
impl std::fmt::Display for WrappedVec<Segment> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for el in self.0.to_owned() {
            writeln!(f, "{}", el)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for WrappedVec<Path> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for el in self.0.to_owned() {
            writeln!(f, "{}", el)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let segments: Vec<Segment> = [
        "FK-gc", "gc-start", "gc-dw", "sp-FN", "dw-end", "FK-start", "dw-gn", "AN-gn", "yh-gn",
        "yh-start", "sp-AN", "ik-dw", "FK-dw", "end-sp", "yh-FK", "gc-gn", "AN-end", "dw-AN",
        "gn-sp", "gn-FK", "sp-FK", "yh-gc",
    ]
    .iter()
    .flat_map(|input_str| {
        let seg = Segment::new(input_str).unwrap();
        [seg.get_reverse(), seg]
    })
    .collect();
    println!("{}", WrappedVec(segments.to_owned()));
    let found_paths = get_paths(&segments);
    println!("{}", found_paths.len());
    // println!("{}", WrappedVec(found_paths));

    Ok(())
}

fn get_paths(segments: &Vec<Segment>) -> Vec<Path> {
    get_paths_rec(
        segments,
        Default::default(),
        Segment::new("null-start").unwrap(),
    )
}

fn get_paths_rec(
    segments: &Vec<Segment>,
    mut previous_path: Path,
    current_segment: Segment,
) -> Vec<Path> {
    if !previous_path.can_add_segment(&current_segment) {
        return vec![];
    }

    previous_path.add_segment(&current_segment);

    if current_segment.end == Cave::new("end") {
        return vec![previous_path];
    }

    let possible_next_segments: Vec<&Segment> =
        get_segments_that_start_with(segments, &current_segment.end).collect();

    return possible_next_segments
        .iter()
        .flat_map(|next_segment| {
            get_paths_rec(
                segments,
                previous_path.to_owned(),
                next_segment.to_owned().to_owned(),
            )
        })
        .collect();
}

fn get_segments_that_start_with<'a>(
    segments: &'a Vec<Segment>,
    cave: &'a Cave,
) -> impl Iterator<Item = &'a Segment> + 'a {
    segments.iter().filter(move |seg| seg.start == *cave)
}
