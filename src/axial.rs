use std::cmp::Ordering;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Axial {
    pub q: i32,
    pub r: i32,
}

impl fmt::Display for Axial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.q, self.r)
    }
}

impl Ord for Axial {
    fn cmp(&self, other: &Axial) -> Ordering {
        match other.q.cmp(&self.q) {
            Ordering::Equal => other.r.cmp(&self.r),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl PartialOrd for Axial {
    fn partial_cmp(&self, other: &Axial) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Axial {
    pub fn new(q: i32, r: i32) -> Axial {
        Axial { q: q, r: r }
    }

    pub fn zero() -> Axial {
        Axial::new(0, 0)
    }

    pub fn vertical_pos(&self) -> i32 {
        self.q + self.r + self.r
    }

    pub fn surrounding_positions(&self) -> Vec<Axial> {
        vec![
            self.north(),
            self.north_east(),
            self.south_east(),
            self.south(),
            self.south_west(),
            self.north_west(),
        ]
    }

    pub fn north(&self) -> Axial {
        Axial::new(self.q, self.r - 1)
    }

    pub fn north_east(&self) -> Axial {
        Axial::new(self.q + 1, self.r - 1)
    }

    pub fn south_east(&self) -> Axial {
        Axial::new(self.q + 1, self.r)
    }

    pub fn south(&self) -> Axial {
        Axial::new(self.q, self.r + 1)
    }

    pub fn south_west(&self) -> Axial {
        Axial::new(self.q - 1, self.r + 1)
    }

    pub fn north_west(&self) -> Axial {
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
