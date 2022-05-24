use crate::{
    axial::Axial,
    tile::{Colour, TileType, Tiles},
};

#[cfg_attr(rustfmt, rustfmt_skip)]
static TEMPLATE: &'static [&'static str] = &[
    "  ____   ",
    " /    \\ ",
    "/  c   \\",
    "\\  x   /",
    " \\____/ "
];

static TEMPLATE_WIDTH: usize = 8;
static TEMPLATE_HEIGHT: usize = 5;
static TEMPLATE_WIDTH_ADD: usize = 6;
static TEMPLATE_HEIGHT_VERT_ADD: usize = 4;
static TEMPLATE_HEIGHT_HORIZ_ADD: usize = 2;

#[derive(Debug)]
pub struct Bounds {
    pub top: Axial,
    pub bottom: Axial,
    pub left: Axial,
    pub right: Axial,
}

impl Bounds {
    fn from_tiles(tiles: &Tiles) -> Self {
        let mut top = &Axial::zero();
        let mut bottom = &Axial::zero();
        let mut left = &Axial::zero();
        let mut right = &Axial::zero();

        tiles.iter().for_each(|(pos, _)| {
            left = if left.q > pos.q { pos } else { left };
            right = if right.q < pos.q { pos } else { right };
            top = if top.vertical_pos() > pos.vertical_pos() {
                pos
            } else {
                top
            };
            bottom = if bottom.vertical_pos() < pos.vertical_pos() {
                pos
            } else {
                bottom
            };
        });

        Bounds {
            top: *top,
            bottom: *bottom,
            left: *left,
            right: *right,
        }
    }

    fn characters_wide(&self) -> usize {
        let width = self.right.q - self.left.q;
        (TEMPLATE_WIDTH as i32 + (width as i32 * TEMPLATE_WIDTH_ADD as i32)) as usize
    }

    fn characters_high(&self) -> usize {
        let qheight = self.bottom.q - self.top.q;
        let rheight = self.bottom.r - self.top.r;
        (TEMPLATE_HEIGHT as i32
            + (qheight as i32 * TEMPLATE_HEIGHT_HORIZ_ADD as i32)
            + (rheight as i32 * TEMPLATE_HEIGHT_VERT_ADD as i32)) as usize
    }
}

pub struct CharacterBuffer2D {
    pub buffer: Vec<char>,

    width: usize,
    height: usize,
}

impl CharacterBuffer2D {
    pub fn new(width: usize, height: usize) -> CharacterBuffer2D {
        let mut this = CharacterBuffer2D {
            buffer: Vec::new(),
            width,
            height,
        };
        this.buffer.resize(width * height, ' ');
        this
    }

    pub fn set(&mut self, x: usize, y: usize, val: char) {
        self.buffer[x + (y * self.width)] = val;
    }

    pub fn set_column(&mut self, col: usize, val: char) {
        for y in 0..self.height {
            self.set(col, y, val);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.buffer[x + (y * self.width)]
    }

    pub fn to_string(&self) -> String {
        self.buffer.clone().into_iter().collect()
    }
}

fn tile_type_to_char(tile_type: TileType) -> char {
    match tile_type {
        TileType::Ant => 'a',
        TileType::Grasshopper => 'g',
        TileType::Beetle => 'b',
        TileType::Queen => 'q',
        TileType::Spider => 's',
    }
}

fn colour_to_char(colour: Colour) -> char {
    match colour {
        Colour::White => 'w',
        Colour::Black => 'b',
    }
}

impl Tiles {
    pub fn render_stdout(&self) {
        macro_rules! get_char {
            ($x:expr, $y:expr) => {
                unsafe {
                    TEMPLATE
                        .get_unchecked($y)
                        .get_unchecked($x..$x + 1)
                        .chars()
                        .next()
                        .unwrap()
                }
            };
        }

        let bounds = Bounds::from_tiles(self);

        let height = bounds.characters_high();
        let width = bounds.characters_wide();
        let mut buffer = CharacterBuffer2D::new(width + 1, height);
        buffer.set_column(width, '\n');

        let mut ordered_tile_positions: Vec<Axial> = Vec::with_capacity(self.0.len());
        self.0.iter().for_each(
            |(pos, tile)| match ordered_tile_positions.binary_search(&pos) {
                Ok(p) | Err(p) => ordered_tile_positions.insert(p, *pos),
            },
        );

        for position in ordered_tile_positions {
            let tile = self.0.get(&position).unwrap();
            let (target_x, target_y) = {
                let target_x = ((position.q - bounds.left.q) as usize) * TEMPLATE_WIDTH_ADD;
                let target_y = TEMPLATE_HEIGHT_HORIZ_ADD
                    * ((position.vertical_pos() - bounds.top.vertical_pos()) as usize);
                (target_x, target_y)
            };
            for source_y in 0..TEMPLATE_HEIGHT {
                for source_x in 0..TEMPLATE_WIDTH {
                    let mut char = get_char!(source_x, source_y);
                    if char == 'x' {
                        char = tile_type_to_char(tile.1);
                    }
                    if char == 'c' {
                        char = colour_to_char(tile.0);
                    }

                    buffer.set(target_x + source_x, target_y + source_y, char);
                }
            }
        }
        println!("{}", buffer.to_string());
    }
}
