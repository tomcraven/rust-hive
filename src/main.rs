#![allow(dead_code)]
#![allow(unused_variables)]
use bit_set::BitSet;
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

    fn zero() -> Self {
        Axial::new(0, 0)
    }

    pub fn surrounding_positions(&self) -> [Axial; 6] {
        [
            self.north(),
            self.north_east(),
            self.south_east(),
            self.south(),
            self.south_west(),
            self.north_west(),
        ]
    }

    pub fn north(&self) -> Axial {
        Axial::new(self.q, self.r - 1)
    }

    pub fn north_east(&self) -> Axial {
        Axial::new(self.q + 1, self.r - 1)
    }

    pub fn south_east(&self) -> Axial {
        Axial::new(self.q + 1, self.r)
    }

    pub fn south(&self) -> Axial {
        Axial::new(self.q, self.r + 1)
    }

    pub fn south_west(&self) -> Axial {
        Axial::new(self.q - 1, self.r + 1)
    }

    pub fn north_west(&self) -> Axial {
        Axial::new(self.q - 1, self.r)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum TileType {
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

struct Tile {
    colour: Colour,
    tile_type: TileType,
}

impl Tile {
    fn ant(colour: Colour) -> Self {
        Tile {
            tile_type: TileType::Ant,
            colour,
        }
    }
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
    // TODO - benchmarks
    // Available placements are stored as a bitset, where 1 represents a free position, and 0 not free
    // the point of this is to be fast, not space efficient (though often they come hand in hand)
    // we want reads and writes to be very fast if we want a speedy MCTS
    top_left: Axial,
    bottom_right: Axial,
    white: BitSet,
    black: BitSet,
}

impl Default for AvailablePlacements {
    fn default() -> Self {
        let white = BitSet::new();
        let black = BitSet::new();

        AvailablePlacements {
            top_left: Axial::zero(),
            bottom_right: Axial::zero(),
            white,
            black,
        }
    }
}

impl AvailablePlacements {
    fn placements_for_colour(&self, colour: &Colour) -> &Vec<Axial> {
        todo!()
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
    fn place(&mut self, position: Axial, tile: Tile) {
        let colour = &tile.colour;

        // Debug assertions
        debug_assert!(self
            .get_available_placements(colour)
            .any(|p| p == &position));

        // Put in the hex grid
        self.grid.put(position, tile);

        self.update_available_placements(position);
    }

    fn update_available_placements(&mut self, position: Axial) {}

    fn get_available_placements(&self, colour: &Colour) -> impl Iterator<Item = &Axial> {
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

    g.place(Axial::new(0, 0), Tile::ant(Colour::White));
    g.place(Axial::new(1, 0), Tile::ant(Colour::Black));

    println!(
        "{:?}",
        g.get_available_placements(&Colour::White)
            .collect::<Vec<&Axial>>()
    );

    let r = StdoutRender::default();
    r.render(&g);
}
