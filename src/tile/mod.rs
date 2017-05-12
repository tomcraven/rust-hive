mod ant;
mod grass_hopper;

use std::rc::Rc;

pub trait Tile {
    fn render(&self);
}

pub fn ant() -> Rc<Tile> {
    Rc::new(ant::Ant {})
}

pub fn grass_hopper() -> Rc<Tile> {
    Rc::new(grass_hopper::GrassHopper {})
}

pub fn standard_game_tiles() -> Vec<Rc<Tile>> {
    vec![ant(), ant(), ant(), grass_hopper(), grass_hopper()]
}
