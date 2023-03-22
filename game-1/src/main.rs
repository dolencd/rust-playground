mod controls;
mod player;
mod position;
mod render;
mod world;

use std::io::{self, Read};

use player::Player;
use world::World;

use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

use crate::{controls::Controls, position::Direction};

fn main() -> io::Result<()> {
    let mut world = World::new();
    let mut player = Player::new();
    let mut stdin = io::stdin();

    let mut termios = Termios::from_fd(0).unwrap();
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &mut termios).unwrap();

    let mut input: [u8; 4] = Default::default();

    loop {
        input.fill(0);
        stdin.read(&mut input).unwrap();
        // println!("{:?}", input);
        let Ok(command): Result<Controls, ()> = input.try_into() else {
            continue;
        };

        match command {
            Controls::MoveUp => player.move_in_direction(&world, Direction::Up),
            Controls::MoveLeft => player.move_in_direction(&world, Direction::Left),
            Controls::MoveDown => player.move_in_direction(&world, Direction::Down),
            Controls::MoveRight => player.move_in_direction(&world, Direction::Right),
            Controls::Esc => break,
            Controls::InventorySlot(e) => player.interact(&mut world, &e),
        }
        println!(
            "\n\n{}",
            render::render(&[&player, &world], &player.position)
        );
    }

    Ok(())
}
