use std::collections::{HashMap, HashSet};

use crate::{
    axial::Axial,
    nav_mesh::NavMesh,
    tile::{Colour, Tile, Tiles},
};

pub struct Board {
    tiles: Tiles,
    free_positions: HashSet<Axial>,
    nav_mesh: NavMesh,
}

impl Default for Board {
    fn default() -> Self {
        let mut ret = Self {
            tiles: Tiles(HashMap::with_capacity(44)), // max tiles for each player
            free_positions: HashSet::with_capacity(32), // TODO consider optimising this
            nav_mesh: NavMesh::default(),
        };
        ret.free_positions.insert(Axial::zero());
        ret
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        panic!("avoid memory allocation at runtime, please use clone_from instead");
    }

    fn clone_from(&mut self, source: &Self) {
        self.tiles.clear();
        self.tiles.extend(source.tiles.iter());

        self.free_positions.clear();
        self.free_positions.extend(source.free_positions.iter());
    }
}

impl Board {
    pub fn place(&mut self, tile: Tile, position: Axial) {
        // first placed tile should be at (0, 0)
        debug_assert!((self.tiles.len() == 0 && position == Axial::zero()) || self.tiles.len() > 0);
        debug_assert!(self.get_available_placements(tile.0).any(|p| p == position));

        // insert tile into our tracked tiles
        self.tiles.insert(position, tile);

        // remove tile from free positions, and update the surrounding new free positions
        self.free_positions.remove(&position);
        self.free_positions.extend(
            position
                .surrounding_positions()
                .iter()
                .filter(|surrounding_position| !self.tiles.contains_key(surrounding_position)),
        );

        // update nav mesh for movements
        self.nav_mesh.update(tile, position);
    }

    pub fn get_available_placements(&self, colour: Colour) -> impl Iterator<Item = Axial> + '_ {
        let adjacent_tiles_all_same_player = move |position: &Axial| match self.tiles.len() {
            1 => true,
            _ => position
                .surrounding_positions()
                .iter()
                .filter_map(|surrounding_position| self.tiles.get(surrounding_position))
                .all(|tile| tile.0 == colour),
        };
        self.free_positions
            .iter()
            .copied()
            .filter(adjacent_tiles_all_same_player)
    }

    pub fn render_stdout(&self) {
        self.tiles.render_stdout();
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::{Colour, TileType};

    use super::*;
    use rand::seq::IteratorRandom;
    use rand::Rng;

    #[test]
    fn place() {
        let mut board = Board::default();

        assert!(!board.tiles.contains_key(&Axial::zero()));
        board.place((Colour::White, TileType::Ant), Axial::zero());
        assert!(board.tiles.contains_key(&Axial::zero()));
    }

    #[test]
    fn clone_from() {
        let mut board = Board::default();
        board.place((Colour::White, TileType::Ant), Axial::zero());
        board.place((Colour::Black, TileType::Beetle), Axial::zero().north());

        let mut cloned_board = Board::default();
        cloned_board.clone_from(&board);

        assert_eq!(board.tiles.len(), cloned_board.tiles.len());
    }

    #[test]
    fn get_available_placements_with_no_tiles_placed() {
        let board = Board::default();

        assert_eq!(
            board
                .get_available_placements(Colour::White)
                .collect::<Vec<Axial>>(),
            [Axial::zero()]
        );
    }

    #[test]
    fn get_available_placements_with_single_tile_placed() {
        let mut board = Board::default();
        board.place((Colour::White, TileType::Ant), Axial::zero());

        assert_eq!(
            board
                .get_available_placements(Colour::White)
                .collect::<HashSet<Axial>>(),
            Axial::zero()
                .surrounding_positions()
                .into_iter()
                .collect::<HashSet<Axial>>()
        );
    }

    /*#[test]
    fn test() {
        for _ in 0..1000 {
            let mut board = Board::default();
            let mut rng = rand::thread_rng();
            let mut colour = Colour::White;
            for _ in 0..20 {
                if rng.gen::<bool>() {
                    board.place(
                        (colour, TileType::Ant),
                        board
                            .get_available_placements(colour)
                            .choose(&mut rng)
                            .unwrap(),
                    );

                    colour = colour.other();
                }
            }

            board.render_stdout();

            let a = board
                .get_available_placements(colour)
                .collect::<HashSet<Axial>>();
            let b = board
                .get_available_placements_v2(colour)
                .collect::<HashSet<Axial>>();

            assert_eq!(a.len(), b.len());
            assert_eq!(a, b);
        }
    }*/
}
