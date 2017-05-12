use position::Position;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Bounds {
    pub top_left: Position,
    pub bottom_right: Position,
}

impl Bounds {
    pub fn new(top_left: Position, bottom_right: Position) -> Bounds {
        Bounds {
            top_left: top_left,
            bottom_right: bottom_right,
        }
    }
}
