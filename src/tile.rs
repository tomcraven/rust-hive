use std::collections::{hash_map::Iter, HashMap};

use crate::axial::Axial;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub fn other(&self) -> Self {
        match self {
            &Colour::White => Colour::Black,
            &Colour::Black => Colour::White,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TileType {
    Ant,
    Grasshopper,
    Beetle,
    Queen,
    Spider,
}

pub type Tile = (Colour, TileType);

pub struct Tiles(pub HashMap<Axial, Tile>);

impl Tiles {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn extend(&mut self, other: Iter<'_, Axial, Tile>) {
        self.0.extend(other);
    }

    pub fn iter(&self) -> Iter<'_, Axial, Tile> {
        self.0.iter()
    }

    pub fn insert(&mut self, k: Axial, v: Tile) -> Option<Tile> {
        self.0.insert(k, v)
    }

    pub fn contains_key(&self, k: &Axial) -> bool {
        self.0.contains_key(k)
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, Axial, Tile> {
        self.0.keys()
    }

    pub fn get(&self, k: &Axial) -> Option<&Tile> {
        self.0.get(k)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}
