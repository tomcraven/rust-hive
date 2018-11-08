use tile::Tile;

pub struct Spider {}

impl Tile for Spider {
    fn render(&self) -> char {
        'S'
    }
}
