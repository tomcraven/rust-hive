use tile::Tile;

pub struct Beetle {}

impl Tile for Beetle {
    fn render(&self) -> char {
        'B'
    }
}
