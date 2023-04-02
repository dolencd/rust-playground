use std::collections::HashMap;

use crate::{
    position::{Direction, Position},
    render::Renderable,
    world::World,
};

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub position: Position,
    pub direction: Direction,
    inventory: HashMap<u8, char>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Position(2, 2),
            direction: Default::default(),
            inventory: Default::default(),
        }
    }

    pub fn move_in_direction(&mut self, world: &World, direction: Direction) {
        let new_position = self.position + direction.into();
        self.direction = direction;
        if world.is_solid(&new_position) {
            return;
        }
        self.position = new_position;
    }

    pub fn interact(&mut self, world: &mut World, inventory_slot: &u8) {
        let slot_occupied = self.inventory.get(inventory_slot).is_some();
        if slot_occupied {
            self.drop_off(world, inventory_slot)
        } else {
            self.pick_up(world, inventory_slot)
        }
    }

    fn pick_up(&mut self, world: &mut World, inventory_slot: &u8) {
        let target_position = self.position + self.direction.into();

        let Some(target_tile) = world.get_square_contents_mut(&target_position) else {
            return;
        };

        if *target_tile == ' ' {
            return;
        }

        if self.inventory.get(inventory_slot).is_some() {
            return;
        }

        self.inventory.insert(*inventory_slot, *target_tile);
        *target_tile = ' ';
    }

    fn drop_off(&mut self, world: &mut World, inventory_slot: &u8) {
        let Some(picked_item) = self.inventory.get(inventory_slot) else {
            return;
        };

        let place_to_drop = self.get_position_in_front();

        let Some(drop_tile) = world.get_square_contents_mut(&place_to_drop) else {
            return;
        };

        if *drop_tile != ' ' {
            return;
        }

        *drop_tile = *picked_item;
        self.inventory.remove(inventory_slot);
    }

    fn get_position_in_front(&self) -> Position {
        self.position + self.direction.into()
    }

    pub fn render_inventory(&self) -> () {
        println!("┏━┳━┳━┳━┳━┳━┳━┳━┳━┳━┓");
        println!("┃1┃2┃3┃4┃5┃6┃7┃8┃9┃0┃");
        println!("┠─╂─╂─╂─╂─╂─╂─╂─╂─╂─┨");
        println!(
            "┃{}┃{}┃{}┃{}┃{}┃{}┃{}┃{}┃{}┃{}┃",
            self.inventory.get(&1).unwrap_or(&' '),
            self.inventory.get(&2).unwrap_or(&' '),
            self.inventory.get(&3).unwrap_or(&' '),
            self.inventory.get(&4).unwrap_or(&' '),
            self.inventory.get(&5).unwrap_or(&' '),
            self.inventory.get(&6).unwrap_or(&' '),
            self.inventory.get(&7).unwrap_or(&' '),
            self.inventory.get(&8).unwrap_or(&' '),
            self.inventory.get(&9).unwrap_or(&' '),
            self.inventory.get(&0).unwrap_or(&' ')
        );
        println!("┗━┻━┻━┻━┻━┻━┻━┻━┻━┻━┛");
    }
}

impl Renderable for Player {
    fn get_at_position(&self, position: &Position) -> Option<&char> {
        if self.position == *position {
            return Some(match self.direction {
                Direction::Down => &'˅',
                Direction::Up => &'˄',
                Direction::Left => &'˂',
                Direction::Right => &'˃',
            });
        }
        None
    }
}
