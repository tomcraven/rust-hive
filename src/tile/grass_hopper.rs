use tile::Tile;

pub struct GrassHopper {}

impl Tile for GrassHopper {
    fn render(&self) -> char {
        'G'
    }
}
