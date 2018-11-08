use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x: x, y: y, z: z }
    }

    pub fn zero() -> Position {
        Position::new(0, 0, 0)
    }
    
    pub fn get_all_adjacent(&self) -> Vec<Position> {
        vec![
            self.north(),
            self.north_east(),
            self.south_east(),
            self.south(),
            self.south_west(),
            self.north_west(),
        ]
    }

    pub fn north(&self) -> Position {
        Position::new(self.x, self.y + 1, self.z - 1)
    }

    pub fn north_east(&self) -> Position {
        Position::new(self.x + 1, self.y, self.z - 1)
    }

    pub fn south_east(&self) -> Position {
        Position::new(self.x + 1, self.y - 1, self.z)
    }

    pub fn south(&self) -> Position {
        Position::new(self.x, self.y - 1, self.z + 1)
    }

    pub fn south_west(&self) -> Position {
        Position::new(self.x - 1, self.y, self.z + 1)
    }

    pub fn north_west(&self) -> Position {
        Position::new(self.x - 1, self.y + 1, self.z)
    }
}

#[cfg(test)]
mod get_adjacent {
    use super::Position;
    use super::Direction::*;

    fn p(pos: (i32, i32, i32)) -> Position {
        let (x, y, z) = pos;
        Position { x: x, y: y, z: z }
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
        adjacent_assert!(origin, (0, 0, 0), Position::north, (0, 1, -1));
        adjacent_assert!(off_centre, (1, -3, 2), Position::north, (1, -2, 1));
    }

    mod north_east {
        adjacent_assert!(origin, (0, 0, 0), Position::north_east, (1, 0, -1));
        adjacent_assert!(off_centre, (1, -3, 2), Position::north_east, (2, -3, 1));
    }

    mod south_east {
        adjacent_assert!(origin, (0, 0, 0), Position::south_east, (1, -1, 0));
        adjacent_assert!(off_centre, (-2, 2, 0), Position::south_east, (-1, 1, 0));
    }

    mod south {
        adjacent_assert!(origin, (0, 0, 0), Position::south, (0, -1, 1));
        adjacent_assert!(orr_centre, (-3, 2, 1), Position::south, (-3, 1, 2));
    }

    mod south_west {
        adjacent_assert!(origin, (0, 0, 0), Position::south_west, (-1, 0, 1));
        adjacent_assert!(off_centre, (1, -1, 0), Position::south_west, (0, -1, 1));
    }

    mod north_west {
        adjacent_assert!(origin, (0, 0, 0), Position::north_west, (-1, 1, 0));
        adjacent_assert!(off_centre, (1, -1, 0), Position::north_west, (0, 0, 0));
    }
}
