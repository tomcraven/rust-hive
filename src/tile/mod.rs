mod ant;
mod beetle;
mod grass_hopper;
mod queen;
mod spider;

use std::rc::Rc;

pub trait Tile {
    fn render(&self) -> char;
}

pub fn ant() -> Rc<Tile> {
    Rc::new(ant::Ant {})
}

pub fn queen() -> Rc<Tile> {
    Rc::new(queen::Queen {})
}

pub fn grass_hopper() -> Rc<Tile> {
    Rc::new(grass_hopper::GrassHopper {})
}

pub fn beetle() -> Rc<Tile> {
    Rc::new(beetle::Beetle {})
}

pub fn spider() -> Rc<Tile> {
    Rc::new(spider::Spider {})
}

pub fn standard_game_tiles() -> Vec<Rc<Tile>> {
    vec![
        queen(),
        beetle(),
        beetle(),
        spider(),
        spider(),
        ant(),
        ant(),
        ant(),
        grass_hopper(),
        grass_hopper(),
        grass_hopper(),
    ]
}
