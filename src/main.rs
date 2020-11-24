extern crate maplit;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Axial {
    q: i32,
    r: i32,
    z: i32,
}

impl Axial {
    fn new(q: i32, r: i32) -> Self {
        Axial { q, r, z: 0 }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Tile {
    Ant,
    Beetle,
    GrassHopper,
    Queen,
    Spider,
}

enum Colour {
    Black,
    White,
}

struct Player {}

impl Default for Player {
    fn default() -> Self {
        Player {}
    }
}

struct HexGrid<T> {
    grid: HashMap<Axial, T>,
}

impl<T> Default for HexGrid<T> {
    fn default() -> Self {
        HexGrid {
            grid: HashMap::new(),
        }
    }
}

impl<T> HexGrid<T> {
    fn put(&mut self, position: Axial, item: T) {
        self.grid.insert(position, item);
    }
}

struct AvailablePlacements {
    white: Vec<Axial>,
    black: Vec<Axial>,
}

impl Default for AvailablePlacements {
    fn default() -> Self {
        AvailablePlacements {
            // TODO: find correct value to use Vec::with_capacity
            white: Vec::new(),
            black: Vec::new(),
        }
    }
}

impl AvailablePlacements {
    fn placements_for_colour(&self, colour: Colour) -> &Vec<Axial> {
        match colour {
            Colour::White => &self.white,
            Colour::Black => &self.black,
        }
    }
}

struct Game {
    grid: HexGrid<Tile>,
    available_placements: AvailablePlacements,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            grid: HexGrid::default(),
            available_placements: AvailablePlacements::default(),
        }
    }
}

impl Game {
    fn place(&mut self, colour: Colour, position: Axial, tile: Tile) {
        // Debug assertions
        debug_assert!(self
            .get_available_placements(colour)
            .any(|p| p == &position));

        // Put in the hex grid
        self.grid.put(position, tile);

        self.update_available_placements(position);
    }

    fn update_available_placements(&mut self, position: Axial) {}

    fn get_available_placements(&self, colour: Colour) -> impl Iterator<Item = &Axial> {
        self.available_placements
            .placements_for_colour(colour)
            .into_iter()
    }
}

trait Render {
    fn render(&self, g: &Game);
}

struct StdoutRender {}

impl Default for StdoutRender {
    fn default() -> Self {
        StdoutRender {}
    }
}

impl Render for StdoutRender {
    fn render(&self, g: &Game) {
        println!("{}", g);
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "test")
    }
}

fn main() {
    let mut g = Game::default();

    g.place(Colour::White, Axial::new(0, 0), Tile::Ant);

    println!(
        "{:?}",
        g.get_available_placements(Colour::White)
            .collect::<Vec<&Axial>>()
    );

    let r = StdoutRender::default();
    r.render(&g);
}
