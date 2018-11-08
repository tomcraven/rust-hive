use std::rc::Rc;
use tile::Tile;
use board::Board;
use axial::Axial;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PlayerNumber {
    One,
    Two,
}

impl PlayerNumber {
    pub fn other(&self) -> PlayerNumber {
        match self {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One
        }
    }
}

pub struct Player {
    tiles: Vec<Rc<Tile>>,
    number: PlayerNumber,
}

impl Player {
    pub fn new(tiles: Vec<Rc<Tile>>, number: PlayerNumber) -> Player {
        return Player { tiles, number };
    }

    pub fn get_tile_placement(&self, board: &Board) -> (Rc<Tile>, Axial) {
        (self.tiles[0].clone(), board.get_possible_tile_placements(self.number)[0])
    }
}
