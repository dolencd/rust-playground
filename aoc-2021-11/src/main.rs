use anyhow::Result;
use grid::Grid;

use crate::grid::Position;
mod grid;

fn main() -> Result<()> {
    let mut grid: Grid = Default::default();

    let p: Position = Position(1, 1);

    let _tmp: Vec<(Position, u8)> = grid.get_adjacent(&p).collect();
    println!("{}", grid);
    // grid.flash_rec(&p);
    // print!("{}", grid);
    for generation in 1..501 {
        let flashes = grid.flash();
        // println!("{}, {}", generation, flashes);
        if flashes == 100 {
            println!("got it!! {}", generation);
        }
        // println!("{}", grid)
    }

    Ok(())
}
