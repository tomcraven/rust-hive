use axial::Axial;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Bounds {
    pub top: Axial,
    pub bottom: Axial,
    pub left: Axial,
    pub right: Axial,
}

impl fmt::Display for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(top: {}, bottom: {}, left: {}, right: {})", 
               self.top.to_string(),
               self.bottom.to_string(),
               self.left.to_string(),
               self.right.to_string())
    }
}

impl Bounds {
    pub fn zero() -> Bounds {
        Bounds::new(Axial::zero(), Axial::zero(), Axial::zero(), Axial::zero())
    }

    pub fn new(top: Axial, bottom: Axial, left: Axial, right: Axial) -> Bounds {
        Bounds {
            top: top,
            bottom: bottom,
            left: left,
            right: right,
        }
    }
}
