use axial::Axial;
use bounds::Bounds;
use placed_tile::PlacedTile;
use player::PlayerNumber;
use render::RenderStdout;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;
use std::iter::once;
use std::iter::repeat;
use std::marker::PhantomData;
use std::rc::Rc;
use tile::Tile;

pub struct Board {
    tiles: HashMap<Axial, Rc<PlacedTile>>,
    render: RenderStdout,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: HashMap::new(),
            render: RenderStdout::new(),
        }
    }

    pub fn place_tile(&mut self, tile: Rc<Tile>, position: Axial, player: PlayerNumber) {
        let placed_tile = Rc::new(PlacedTile::new(tile, position, player));
        self.tiles.insert(position, placed_tile.clone());
        self.render.push(placed_tile.clone());
    }

    pub fn render(&self) {
        self.render.render();
    }

    pub fn get_possible_tile_placements(&self, player: PlayerNumber) -> Vec<Axial> {
        let position_is_free = |position: &Axial| !self.tiles.contains_key(position);

        let adjacent_tiles_all_same_player = |position: &Axial| {
            position
                .surrounding_positions()
                .iter()
                .filter_map(|surrounding_position| self.tiles.get(surrounding_position))
                .all(|placed_tile| placed_tile.player == player)
        };

        match self.tiles.len() {
            0 => vec![Axial::zero()],
            1 => Axial::zero().surrounding_positions(),
            _ => {
                let mut tiles = self
                    .tiles
                    .iter()
                    .flat_map(|(position, tile)| position.surrounding_positions())
                    .filter(position_is_free)
                    .filter(adjacent_tiles_all_same_player)
                    .collect::<Vec<_>>();
                tiles.sort();
                tiles.dedup();
                tiles
            }
        }
    }
}

#[cfg(test)]
mod helpers {
    use super::*;
    use axial::Axial;
    use tile::ant;

    pub fn board_with_tiles_at(positions: Vec<Axial>) -> Board {
        let mut b = Board::new();
        let mut current_player = PlayerNumber::One;
        for pos in positions {
            b.place_tile(ant(), pos, current_player);
            current_player = current_player.other();
        }
        b
    }
}

#[cfg(test)]
mod get_possible_tile_placements {
    use super::*;

    fn get_placements(positions: Vec<Axial>, player: PlayerNumber) -> Vec<Axial> {
        let b = helpers::board_with_tiles_at(positions);
        b.get_possible_tile_placements(player)
    }

    #[test]
    fn empty_board() {
        let b = Board::new();
        assert_eq!(1, b.get_possible_tile_placements(PlayerNumber::One).len());
        assert_eq!(
            Axial::zero(),
            b.get_possible_tile_placements(PlayerNumber::One)[0]
        );
    }

    #[test]
    fn single_tile() {
        let placements = get_placements(vec![Axial::zero()], PlayerNumber::Two);
        assert_eq!(6, placements.len());
    }

    #[test]
    fn two_tiles() {
        let placements = get_placements(
            vec![Axial::zero(), Axial::zero().south()],
            PlayerNumber::One,
        );

        assert_eq!(3, placements.len());
        assert_eq!(true, placements.contains(&Axial::zero().north_west()));
        assert_eq!(true, placements.contains(&Axial::zero().north()));
        assert_eq!(true, placements.contains(&Axial::zero().north_east()));
    }

    #[test]
    fn two_tiles_other_player() {
        let placements = get_placements(
            vec![Axial::zero(), Axial::zero().south()],
            PlayerNumber::Two,
        );

        assert_eq!(3, placements.len());
        assert_eq!(
            true,
            placements.contains(&Axial::zero().south().south_west())
        );
        assert_eq!(true, placements.contains(&Axial::zero().south().south()));
        assert_eq!(
            true,
            placements.contains(&Axial::zero().south().south_east())
        );
    }

    #[test]
    fn three_tiles() {
        let placements = get_placements(
            vec![Axial::zero(), Axial::zero().south(), Axial::zero().north()],
            PlayerNumber::One,
        );

        assert_eq!(5, placements.len());
        assert_eq!(true, placements.contains(&Axial::zero().north_west()));
        assert_eq!(true, placements.contains(&Axial::zero().north_east()));
        assert_eq!(
            true,
            placements.contains(&Axial::zero().north().north_west())
        );
        assert_eq!(true, placements.contains(&Axial::zero().north().north()));
        assert_eq!(
            true,
            placements.contains(&Axial::zero().north().north_east())
        );
    }
}
