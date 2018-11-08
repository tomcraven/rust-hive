use tile::Tile;

pub struct Queen {}

impl Tile for Queen {
    fn render(&self) -> char {
        'Q'
    }
}
