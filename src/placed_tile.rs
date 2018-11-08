use std::rc::Rc;
use axial::Axial;
use player::PlayerNumber;
use tile::Tile;
use std::cmp::Ordering;
use std::cmp::Eq;

pub struct PlacedTile {
    pub tile: Rc<Tile>,
    pub player: PlayerNumber,
    pub position: Axial,
}

impl PlacedTile {
    pub fn new(tile: Rc<Tile>, position: Axial, player: PlayerNumber) -> PlacedTile {
        PlacedTile {
            tile: tile,
            player: player,
            position: position,
        }
    }
}

impl Ord for PlacedTile {
    fn cmp(&self, other: &PlacedTile) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for PlacedTile {
    fn partial_cmp(&self, other: &PlacedTile) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl PartialEq for PlacedTile {
    fn eq(&self, other: &PlacedTile) -> bool {
        self.position == other.position
    }
}

impl Eq for PlacedTile {}
