#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }

    pub fn get_all_adjacent(&self) -> Vec<Position> {
        vec![self.north(),
             self.north_east(),
             self.south_east(),
             self.south(),
             self.south_west(),
             self.north_west()]
    }

    pub fn north(&self) -> Position {
        Position::new(self.x, self.y - 2)
    }

    pub fn north_east(&self) -> Position {
        Position::new(if self.y % 2 == 0 { self.x } else { self.x + 1 },
                      self.y - 1)
    }

    pub fn south_east(&self) -> Position {
        Position::new(if self.y % 2 == 0 { self.x } else { self.x + 1 },
                      self.y + 1)
    }

    pub fn south(&self) -> Position {
        Position::new(self.x, self.y + 2)
    }

    pub fn south_west(&self) -> Position {
        Position::new(if self.y % 2 == 0 { self.x - 1 } else { self.x },
                      self.y + 1)
    }

    pub fn north_west(&self) -> Position {
        Position::new(if self.y % 2 == 0 { self.x - 1 } else { self.x },
                      self.y - 1)
    }
}


#[cfg(test)]
mod get_adjacent {
    use super::Position;
    use super::Direction::*;

    fn p(pos: (i32, i32)) -> Position {
        let (x, y) = pos;
        Position { x: x, y: y }
    }

    macro_rules! adjacent_assert {
        ($name:ident, $starting_position:expr, $direction:expr, $expected:expr) => {
            #[test]
            fn $name() {
                use position::Position;
                use super::p;
                assert_eq!($direction(&p($starting_position)),
                           p($expected));
            }
        }
    }

    mod north {
        adjacent_assert!(origin, (0, 0), Position::north, (0, -2));
        adjacent_assert!(off_centre, (3, 3), Position::north, (3, 1));
    }

    mod north_east {
        adjacent_assert!(origin, (0, 0), Position::north_east, (0, -1));
        adjacent_assert!(off_centre, (0, 1), Position::north_east, (1, 0));
    }

    mod south_east {
        adjacent_assert!(origin, (0, 0), Position::south_east, (0, 1));
        adjacent_assert!(off_centre, (0, 1), Position::south_east, (1, 2));
    }

    mod south {
        adjacent_assert!(origin, (0, 0), Position::south, (0, 2));
        adjacent_assert!(orr_centre, (3, 1), Position::south, (3, 3));
    }

    mod south_west {
        adjacent_assert!(origin, (0, 0), Position::south_west, (-1, 1));
        adjacent_assert!(off_centre, (0, 1), Position::south_west, (0, 2));
    }

    mod north_west {
        adjacent_assert!(origin, (0, 0), Position::north_west, (-1, -1));
        adjacent_assert!(off_centre, (0, 1), Position::north_west, (0, 0));
    }
}
