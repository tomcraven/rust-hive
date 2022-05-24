use std::cmp::Ordering;
use std::fmt;

pub type Coord = i8;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Axial {
    pub q: Coord,
    pub r: Coord,
}

impl fmt::Display for Axial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.q, self.r)
    }
}

impl Ord for Axial {
    fn cmp(&self, other: &Axial) -> Ordering {
        match other.r.cmp(&self.r) {
            Ordering::Equal => other.q.cmp(&self.q),
            v @ _ => v,
        }
    }
}

impl PartialOrd for Axial {
    fn partial_cmp(&self, other: &Axial) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Axial {
    pub const fn new(q: Coord, r: Coord) -> Axial {
        Axial { q, r }
    }

    pub const fn zero() -> Axial {
        Axial::new(0, 0)
    }

    pub const fn vertical_pos(&self) -> Coord {
        self.q + self.r + self.r
    }

    pub const fn surrounding_positions(&self) -> [Axial; 6] {
        [
            self.north(),
            self.north_east(),
            self.south_east(),
            self.south(),
            self.south_west(),
            self.north_west(),
        ]
    }

    pub const fn north(&self) -> Axial {
        Axial::new(self.q, self.r - 1)
    }

    pub const fn north_east(&self) -> Axial {
        Axial::new(self.q + 1, self.r - 1)
    }

    pub const fn south_east(&self) -> Axial {
        Axial::new(self.q + 1, self.r)
    }

    pub const fn south(&self) -> Axial {
        Axial::new(self.q, self.r + 1)
    }

    pub const fn south_west(&self) -> Axial {
        Axial::new(self.q - 1, self.r + 1)
    }

    pub const fn north_west(&self) -> Axial {
        Axial::new(self.q - 1, self.r)
    }
}

#[cfg(test)]
mod surrounding_tiles {
    use super::*;

    #[test]
    fn at_zero() {
        let a = Axial::zero();
        let tiles = a.surrounding_positions();
        assert_eq!(tiles.len(), 6);
    }
}
