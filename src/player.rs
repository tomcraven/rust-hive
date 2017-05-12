use std::rc::Rc;
use tile::Tile;
use board::BoardProxy;
use position::Position;

#[derive(Clone, Copy)]
pub enum PlayerNumber {
    One,
    Two,
}

pub struct Player {
    tiles: Vec<Rc<Tile>>,
}

impl Player {
    pub fn new(tiles: Vec<Rc<Tile>>) -> Player {
        return Player { tiles: tiles };
    }

    pub fn get_tile_placement(&self, board: BoardProxy) -> (Rc<Tile>, Position) {
        (self.tiles[0].clone(), board.get_possible_tile_placements()[0])
    }
}
