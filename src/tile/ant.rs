use tile::Tile;

pub struct Ant {}

impl Tile for Ant {
    fn render(&self) {
        println!("A");
    }
}
