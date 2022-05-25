use itertools::Itertools;
use std::collections::HashSet;
use std::{borrow::BorrowMut, cell::RefCell};

use crate::{
    axial::Axial,
    tile::{Tile, Tiles},
};

pub struct NavMesh {
    can_move_scratch: HashSet<Axial>,
}

impl Default for NavMesh {
    fn default() -> Self {
        Self {
            can_move_scratch: HashSet::with_capacity(22),
        }
    }
}

pub fn path_between(
    tiles: &Tiles,
    a: &Axial,
    b: &Axial,
    scratch: &mut HashSet<Axial>,
) -> Option<Box<dyn Iterator<Item = Axial>>> {
    use std::iter;

    //std::thread::sleep(std::time::Duration::from_secs(1));

    println!("begin {:?} {:?}", a, b);

    if a == b {
        println!("a");
        Some(Box::new(iter::empty::<Axial>()))
    } else {
        println!("b {:?}", a);

        scratch.insert(*a);
        for p in a
            .surrounding_positions()
            .into_iter()
            .filter(|p| tiles.contains_key(p))
            .filter(|p| !scratch.contains(p))
        {
            println!("c {:?} {:?}", a, p);
            if let Some(further) = path_between(tiles, &p, b, scratch) {
                println!("d");
                return Some(Box::new(iter::once(p).chain(further)));
            } else {
                println!("e");
                //return None;
            }
        }

        println!("f");
        None
    }
}

impl NavMesh {
    pub fn update(&mut self, tile: Tile, position: Axial) {}

    pub fn can_move_tile(&mut self, tiles: &Tiles, position: Axial) -> bool {
        debug_assert!(tiles.contains_key(&position));

        thread_local!(static SCRATCH: RefCell<HashSet<Axial>> = RefCell::new(HashSet::with_capacity(22)));

        // TODO - do we need edge cases for 0, 1, and 2 tiles in array?

        SCRATCH.with(|scratch| {
            let surrounding_positions = position.surrounding_positions();
            for (n, a) in surrounding_positions.iter().enumerate() {
                for b in surrounding_positions.iter().skip(n + 1) {
                    // can a route to b?
                    // - if all of them can then the island is connected
                    // - if one pair can't then island is disconnected and return false

                    scratch.borrow_mut().clear();

                    if let Some(path) = path_between(tiles, a, b, &mut scratch.borrow_mut()) {
                        println!("{:?}", path.collect::<Vec<Axial>>());
                    } else {
                        println!("no path between {:?} and {:?}", a, b);
                        return false;
                    }
                }
            }

            true
        })
    }
}

#[cfg(test)]
mod can_move_tile {
    use crate::tile::{Colour, TileType};

    use super::*;

    use alloc_counter::AllocCounterSystem;

    #[global_allocator]
    static A: AllocCounterSystem = AllocCounterSystem;

    #[test]
    fn single_tile() {
        let mut tiles = Tiles::with_capacity(16);
        tiles.insert(Axial::zero(), (Colour::White, TileType::Ant));

        let mut nav_mesh = NavMesh::default();
        assert!(!nav_mesh.can_move_tile(&tiles, Axial::zero()));
    }

    #[test]
    fn two_tiles() {
        let mut tiles = Tiles::with_capacity(16);
        tiles.insert(Axial::zero(), (Colour::White, TileType::Ant));
        tiles.insert(Axial::zero().north(), (Colour::Black, TileType::Ant));

        let mut nav_mesh = NavMesh::default();
        assert!(!nav_mesh.can_move_tile(&tiles, Axial::zero()));
        assert!(!nav_mesh.can_move_tile(&tiles, Axial::zero().north()));
    }

    #[test]
    fn three_tiles() {
        let mut tiles = Tiles::with_capacity(16);
        tiles.insert(Axial::zero(), (Colour::White, TileType::Ant));
        tiles.insert(Axial::zero().north(), (Colour::Black, TileType::Ant));
        tiles.insert(Axial::zero().south(), (Colour::White, TileType::Ant));

        let mut nav_mesh = NavMesh::default();

        nav_mesh.can_move_tile(&tiles, Axial::zero().south());

        //assert!(!nav_mesh.can_move_tile(&tiles, Axial::zero()));
        //assert!(!nav_mesh.can_move_tile(&tiles, Axial::zero().north()));
        //assert!(nav_mesh.can_move_tile(&tiles, Axial::zero().south()));
    }

    #[test]
    fn test_allocations() {
        let mut tiles = Tiles::with_capacity(16);
        tiles.insert(Axial::zero(), (Colour::White, TileType::Ant));
        tiles.insert(Axial::zero().north(), (Colour::Black, TileType::Ant));
        tiles.insert(Axial::zero().south(), (Colour::White, TileType::Ant));

        let mut nav_mesh = NavMesh::default();
        let (count, _) =
            alloc_counter::count_alloc(|| nav_mesh.can_move_tile(&tiles, Axial::zero()));

        println!("{:?}", count);
    }
}
