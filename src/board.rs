use position::Position;
use std::rc::Rc;
use tile::Tile;
use player::PlayerNumber;
use std::cmp;
use std::iter;
use bounds::Bounds;
use std::marker::PhantomData;
use std::iter::repeat;
use std::iter::once;

struct TileWrapper {
    tile: Rc<Tile>,
    player: PlayerNumber,
    position: Position,
}

pub struct Board {
    tiles: Vec<TileWrapper>,
    bounds: Option<Bounds>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: Vec::new(),
            bounds: None,
        }
    }

    pub fn place_tile(&mut self, tile: Rc<Tile>, position: Position, player: PlayerNumber) {
        self.tiles.push(TileWrapper {
            tile: tile,
            position: position,
            player: player,
        });
        self.update_bounds(position);
    }

    pub fn render(&self) {}

    fn get_insert_position(&self, position: Position) -> usize {
        let result = self.tiles
            .iter()
            .position(|t| {
                let tile_pos = t.position;
                (tile_pos.y > position.y) || (tile_pos.y == position.y && tile_pos.x > position.x)
            });
        match result {
            Some(x) => x,
            None => self.tiles.len(),
        }
    }

    fn update_bounds(&mut self, pos: Position) {
        match self.bounds {
            None => {
                self.bounds = Some(Bounds::new(pos, pos));
            }
            Some(ref mut b) => {
                b.top_left.x = cmp::min(b.top_left.x, pos.x);
                b.top_left.y = cmp::min(b.top_left.y, pos.y);
                b.bottom_right.x = cmp::max(b.bottom_right.x, pos.x);
                b.bottom_right.y = cmp::max(b.bottom_right.y, pos.y);
            }
        }
    }

    fn tiles_between_positions(&self, a: Position, b: Position) -> usize {
        let board_width = match self.bounds {
            None => (a.x - b.y).abs(),
            Some(b) => b.bottom_right.x - b.top_left.x
        };
        ((a.x - b.x).abs() + ((a.y - b.y).abs() * board_width)) as usize
    }

    fn num_blank_tiles(&self, previous: Option<Position>, next: Position) -> usize {
        let ret = match previous {
            None => match self.bounds {
                None => 0,
                Some(b) => self.tiles_between_positions(b.top_left, next),
            },
            Some(p) => self.tiles_between_positions(p, next)
        };
        println!("num tiles between {:?} and {:?}\nwith bounds {:?} is {}", 
                 previous, next, self.bounds, ret);
        return ret;
    }
    
    fn last_tile_position(&self) -> Option<Position> {
        match self.tiles.last() {
            None => None,
            Some(t) => Some(t.position)
        }
    }

    fn get_tiles<'a>(&'a self) -> Vec<Option<&'a TileWrapper>> {
        let mut previous_position: Option<Position> = None;
        self.tiles
            .iter()
            .flat_map(move |a| {
                let ret = repeat(None)
                    .take(self.num_blank_tiles(previous_position, a.position))
                    .chain(once(Some(a)));
                previous_position = Some(a.position);
                ret
            })
            .chain(repeat(None).take(
                match self.bounds {
                    None => 0,
                    Some(b) => self.num_blank_tiles(
                        self.last_tile_position(),
                        b.bottom_right)
                }))
            .collect::<Vec<Option<&'a TileWrapper>>>()
    }
}

pub struct BoardProxy<'a> {
    board: &'a mut Board,
}

impl<'a> BoardProxy<'a> {
    pub fn new(board: &mut Board) -> BoardProxy {
        BoardProxy { board: board }
    }

    pub fn get_possible_tile_placements(&self) -> Vec<Position> {
        vec![Position { x: 0, y: 0 }]
    }
}

#[cfg(test)]
mod helpers {
    use super::*;
    use position::Position;
    use tile::ant;

    pub fn board_with_tiles_at(positions: Vec<Position>) -> Board {
        let mut b = Board::new();
        for pos in positions {
            b.place_tile(ant(), pos, PlayerNumber::One);
        }
        b
    }
}

#[cfg(test)]
mod update_bounds {
    use super::*;

    #[test]
    fn empty_board() {
        let mut b = Board::new();
        let pos = Position::new(10, 10);
        b.update_bounds(pos);
        assert_eq!(b.bounds.unwrap().top_left, pos);
        assert_eq!(b.bounds.unwrap().bottom_right, pos);
    }

    #[test]
    fn single_pos() {
        let mut b = Board::new();
        b.update_bounds(Position::new(1, 0));

        let top_left = Position::new(-1, -1);
        let bottom_right = Position::new(1, 1);
        b.update_bounds(top_left);
        b.update_bounds(bottom_right);

        assert_eq!(b.bounds.unwrap().top_left, top_left);
        assert_eq!(b.bounds.unwrap().bottom_right, bottom_right);
    }

    #[test]
    fn no_update_if_inside_bounds() {
        let mut b = Board::new();
        let top_left = Position::new(-1, -1);
        let bottom_right = Position::new(1, 1);
        b.update_bounds(top_left);
        b.update_bounds(bottom_right);
        assert_eq!(b.bounds.unwrap().top_left, top_left);
        assert_eq!(b.bounds.unwrap().bottom_right, bottom_right);

        b.update_bounds(Position::new(0, 0));
        assert_eq!(b.bounds.unwrap().top_left, top_left);
        assert_eq!(b.bounds.unwrap().bottom_right, bottom_right);
    }
}

#[cfg(test)]
mod get_insert_position {
    use super::*;
    use tile::*;

    #[test]
    fn empty() {
        let b = helpers::board_with_tiles_at(vec![]);
        assert_eq!(0, b.get_insert_position(Position::new(0, 0)));
    }

    #[test]
    fn single_at_origin() {
        let origin = Position::new(0, 0);
        let b = helpers::board_with_tiles_at(vec![origin]);
        assert_eq!(0, b.get_insert_position(origin.north()));
        assert_eq!(0, b.get_insert_position(origin.north_east()));
        assert_eq!(0, b.get_insert_position(origin.north_west()));
        assert_eq!(1, b.get_insert_position(origin.south()));
        assert_eq!(1, b.get_insert_position(origin.south_east()));
        assert_eq!(1, b.get_insert_position(origin.south_west()));
    }

    #[test]
    fn two_above_and_below() {
        let origin = Position::new(0, 0);
        let b = helpers::board_with_tiles_at(vec![origin, origin.south()]);
        assert_eq!(1, b.get_insert_position(origin.south_east()));
        assert_eq!(1, b.get_insert_position(origin.south_west()));
        assert_eq!(2, b.get_insert_position(origin.south().south_east()));
        assert_eq!(2, b.get_insert_position(origin.south().south_west()));
    }
}

#[cfg(test)]
mod get_tiles {
    use super::*;

    #[test]
    fn returns_empty_iterator_when_no_tiles() {
        let b = helpers::board_with_tiles_at(vec![]);
        assert!(b.get_tiles().iter().next().is_none());
    }

    #[test]
    fn returns_single_tile() {
        let b = helpers::board_with_tiles_at(vec![Position::new(0, 0)]);
        let tiles = b.get_tiles();
        let t = tiles.iter().next();
        assert!(!t.is_none());
    }

    #[test]
    fn returns_blank_then_single_tile() {
        let mut b = helpers::board_with_tiles_at(vec![Position::new(0, 0)]);
        b.bounds.as_mut().unwrap().top_left.x -= 1;
        let tiles = b.get_tiles();
        let mut iter = tiles.iter();
        let first_tile = iter.next();
        let second_tile = iter.next();
        let third_tile = iter.next();

        assert!(first_tile.unwrap().is_none());
        assert!(!second_tile.unwrap().is_none());
        assert!(third_tile.is_none());
    }
    
    #[test]
    fn returns_single_tile_then_blank() {
        let mut b = helpers::board_with_tiles_at(vec![Position::new(0, 0)]);
        b.bounds.as_mut().unwrap().bottom_right.x += 1;
        let tiles = b.get_tiles();
        let mut iter = tiles.iter();
        let first_tile = iter.next();
        let second_tile = iter.next();
        let third_tile = iter.next();

        assert!(!first_tile.unwrap().is_none());
        assert!(second_tile.unwrap().is_none());
        assert!(third_tile.is_none());
    }
    
    #[test]
    fn returns_tile_then_blank_then_tile() {
        let b = helpers::board_with_tiles_at(
            vec![Position::new(0, 0), Position::new(2, 0)]
        );
        let tiles = b.get_tiles();
        let mut iter = tiles.iter();
        let first_tile = iter.next();
        let second_tile = iter.next();
        let third_tile = iter.next();
        let fourth_tile = iter.next();

        assert!(!first_tile.unwrap().is_none());
        assert!(second_tile.unwrap().is_none());
        assert!(!third_tile.unwrap().is_none());
        assert!(fourth_tile.is_none());
    }    
}

#[cfg(test)]
mod tiles_between_positions {
    use super::*;
    use bounds::Bounds;
    
    
    fn create_bounded_board() -> Board {
        /*
        
        Board looks like this:
        
          00112233
        0 x x x x
        1  x x x x
        2 x x x x
        3  x x x x
        4 x x x x
        5  x x x x      
                      
        */
        
        let mut b = Board::new();
        b.bounds = Some(Bounds {
            top_left: Position::new(0, 0),
            bottom_right: Position::new(3, 5),
        });
        b
    }

    #[test]
    fn panics_when_positions_are_equal() {
        let b = Board::new();
        let tiles_between = b.tiles_between_positions(Position::new(0, 0), Position::new(0, 0));
        assert_eq!(0, tiles_between);
    }

    #[test]
    fn returns_0_when_tiles_are_next_to_each_other_horizontally() {
        let b = Board::new();
        let tiles_between = b.tiles_between_positions(Position::new(0, 0), Position::new(1, 0));
        assert_eq!(1, tiles_between);
    }

    #[test]
    fn when_tiles_are_ontop_of_each_other() {
        /*
        
          00112233
        0 x o 1 2
        1  3 o x x
        2 x x x x
        3  x x x x
        4 x x x x
        5  x x x x
        
        */
    
        let b = create_bounded_board();
        let tiles_between = b.tiles_between_positions(Position::new(1, 0), Position::new(1, 1));
        assert_eq!(3, tiles_between);
    }
}
