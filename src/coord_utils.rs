use position::Position;
use axial::Axial;

pub fn position_to_axial(position: Position) -> Axial {
    Axial::new(position.x, position.z)
}

pub fn axial_to_position(axial: Axial) -> Position {
    Position::new(axial.q, -axial.q - axial.r, axial.r)
}
