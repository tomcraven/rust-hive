use crate::{axial::Axial, tile::Tile};

pub struct NavMesh {}

impl Default for NavMesh {
    fn default() -> Self {
        Self {}
    }
}
impl NavMesh {
    pub fn update(&mut self, tile: Tile, position: Axial) {}
}
