use axial::Axial;
use bounds::Bounds;
use character_buffer_2d::CharacterBuffer2D;
use coord_utils;
use placed_tile::PlacedTile;
use player::PlayerNumber;
use std::cmp;
use std::collections::HashMap;
use std::convert;
use std::rc::Rc;
use tile;
use tile::ant;

#[cfg_attr(rustfmt, rustfmt_skip)]
static TEMPLATE: &'static [&'static str] = &[
    "  ____   ",
    " /    \\ ",
    "/  x   \\",
    "\\      /",
    " \\____/ "
];

/*

max width * max height * num diff pieces * potential beetle height

not sure how to calculate width and height, will need to research how to store hex coords in a 2d
array, can assume it'll be num tiles per player * 2 for now...

22 * 22 * 5 * 5 = 12,100

fak

*/

static TEMPLATE_WIDTH: usize = 8;
static TEMPLATE_HEIGHT: usize = 5;
static TEMPLATE_WIDTH_ADD: usize = 6;
static TEMPLATE_HEIGHT_VERT_ADD: usize = 4;
static TEMPLATE_HEIGHT_HORIZ_ADD: usize = 2;

pub struct RenderStdout {
    bounds: Option<Bounds>,
    pieces: Vec<Rc<PlacedTile>>,
}

impl RenderStdout {
    pub fn new() -> RenderStdout {
        RenderStdout {
            bounds: None,
            pieces: Vec::new(),
        }
    }

    pub fn push(&mut self, tile: Rc<PlacedTile>) {
        self.update_bounds(tile.position);
        match self.pieces.binary_search(&tile) {
            Ok(pos) | Err(pos) => self.pieces.insert(pos, tile.clone()),
        }
    }

    pub fn render(&self) {
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

        match self.bounds {
            None => {}
            Some(bounds) => {
                let height = self.characters_high();
                let width = self.characters_wide();
                let mut buffer = CharacterBuffer2D::new(width + 1, height);
                buffer.set_column(width, '\n');
                for piece in &self.pieces {
                    let (target_x, target_y) = self.render_position(&piece.position);
                    for source_y in 0..TEMPLATE_HEIGHT {
                        for source_x in 0..TEMPLATE_WIDTH {
                            let mut char = get_char!(source_x, source_y);
                            if char == 'x' {
                                char = piece.tile.render();
                            }

                            buffer.set(target_x + source_x, target_y + source_y, char);
                        }
                    }
                }
                println!("{}", buffer.to_string());
            }
        }
    }

    fn render_position(&self, position: &Axial) -> (usize, usize) {
        assert!(self.bounds.is_some(), "bounds must be present");
        let bounds = self.bounds.unwrap();
        let target_x = ((position.q - bounds.left.q) as usize) * TEMPLATE_WIDTH_ADD;
        let target_y = TEMPLATE_HEIGHT_HORIZ_ADD
            * ((position.vertical_pos() - bounds.top.vertical_pos()) as usize);
        (target_x, target_y)
    }

    fn update_bounds(&mut self, pos: Axial) {
        match self.bounds {
            None => self.bounds = Some(Bounds::new(pos, pos, pos, pos)),
            Some(ref mut bounds) => {
                bounds.left = if bounds.left.q > pos.q {
                    pos
                } else {
                    bounds.left
                };
                bounds.right = if bounds.right.q < pos.q {
                    pos
                } else {
                    bounds.right
                };
                bounds.top = if bounds.top.vertical_pos() > pos.vertical_pos() {
                    pos
                } else {
                    bounds.top
                };
                bounds.bottom = if bounds.bottom.vertical_pos() < pos.vertical_pos() {
                    pos
                } else {
                    bounds.bottom
                };
            }
        }
    }

    fn characters_wide(&self) -> usize {
        match self.bounds {
            None => 0,
            Some(bounds) => {
                let width = bounds.right.q - bounds.left.q;
                (TEMPLATE_WIDTH as i32 + (width * TEMPLATE_WIDTH_ADD as i32)) as usize
            }
        }
    }

    fn characters_high(&self) -> usize {
        match self.bounds {
            None => 0,
            Some(bounds) => {
                let qheight = bounds.bottom.q - bounds.top.q;
                let rheight = bounds.bottom.r - bounds.top.r;
                (TEMPLATE_HEIGHT as i32
                    + (qheight * TEMPLATE_HEIGHT_HORIZ_ADD as i32)
                    + (rheight * TEMPLATE_HEIGHT_VERT_ADD as i32)) as usize
            }
        }
    }
}

#[cfg(test)]
mod helpers {
    use super::*;

    pub fn render_with_tiles_at_positions(positions: Vec<Axial>) -> RenderStdout {
        let mut r = RenderStdout::new();
        for position in positions {
            r.push(Rc::new(PlacedTile::new(ant(), position, PlayerNumber::One)));
        }
        r
    }
}

#[cfg(test)]
mod render {
    use super::helpers::*;
    use super::*;

    #[test]
    fn empty_board() {
        let r = render_with_tiles_at_positions(vec![]);
        assert_eq!("", r.render());
    }

    #[test]
    fn single_tile() {
        let r = render_with_tiles_at_positions(vec![Axial::zero()]);
        assert_eq!(
            format!(
                "{}\n",
                [
                    r#" \____/ "#,
                    r#" /    \ "#,
                    r#"/      \"#,
                    r#"\      /"#,
                    r#" \____/ "#
                ].join("\n")
            ),
            r.render()
        );
    }

    #[test]
    fn single_tile_off_centre() {
        let r = render_with_tiles_at_positions(vec![Axial::zero().south_west()]);
        assert_eq!(
            format!(
                "{}\n",
                [
                    r#" \____/ "#,
                    r#" /    \ "#,
                    r#"/      \"#,
                    r#"\      /"#,
                    r#" \____/ "#
                ].join("\n")
            ),
            r.render()
        );
    }

    #[test]
    fn two_tiles_vertical() {
        let r = render_with_tiles_at_positions(vec![Axial::zero(), Axial::zero().south()]);
        assert_eq!(
            format!(
                "{}\n",
                [
                    r#" \____/ "#,
                    r#" /    \ "#,
                    r#"/      \"#,
                    r#"\      /"#,
                    r#" \____/ "#,
                    r#" /    \ "#,
                    r#"/      \"#,
                    r#"\      /"#,
                    r#" \____/ "#
                ].join("\n")
            ),
            r.render()
        );
    }

    #[test]
    fn three_tiles_vertical() {
        let r = render_with_tiles_at_positions(vec![
            Axial::zero(),
            Axial::zero().south(),
            Axial::zero().south().south(),
        ]);
        assert_eq!(
            format!(
                "{}\n",
                [
                    r#" \____/ "#,
                    r#" /    \ "#,
                    r#"/      \"#,
                    r#"\      /"#,
                    r#" \____/ "#,
                    r#" /    \ "#,
                    r#"/      \"#,
                    r#"\      /"#,
                    r#" \____/ "#,
                    r#" /    \ "#,
                    r#"/      \"#,
                    r#"\      /"#,
                    r#" \____/ "#
                ].join("\n")
            ),
            r.render()
        );
    }

    #[test]
    fn two_tiles_horizontal_1() {
        let r = render_with_tiles_at_positions(vec![Axial::zero(), Axial::zero().south_east()]);
        assert_eq!(
            format!(
                "{}\n",
                [
                    r#" \____/       "#,
                    r#" /    \       "#,
                    r#"/      \____/ "#,
                    r#"\      /    \ "#,
                    r#" \____/      \"#,
                    r#"      \      /"#,
                    r#"       \____/ "#,
                ].join("\n")
            ),
            r.render()
        );
    }

    #[test]
    fn two_tiles_horizontal_2() {
        let r = render_with_tiles_at_positions(vec![Axial::zero(), Axial::zero().south_west()]);
        assert_eq!(
            format!(
                "{}\n",
                [
                    r#"       \____/ "#,
                    r#"       /    \ "#,
                    r#" \____/      \"#,
                    r#" /    \      /"#,
                    r#"/      \____/ "#,
                    r#"\      /      "#,
                    r#" \____/       "#,
                ].join("\n")
            ),
            r.render()
        );
    }
}

#[cfg(test)]
mod render_position {
    use super::helpers::*;
    use super::*;

    #[test]
    fn two_tile_horizontally() {
        let r = render_with_tiles_at_positions(vec![Axial::zero(), Axial::zero().south_west()]);
        assert_eq!(
            (0, TEMPLATE_HEIGHT_HORIZ_ADD),
            r.render_position(Axial::zero().south_west())
        );

        assert_eq!((TEMPLATE_WIDTH_ADD, 0), r.render_position(Axial::zero()));
    }
}

#[cfg(test)]
mod character_dimensions {
    use super::helpers::*;
    use super::*;

    static TEST_HEIGHT: bool = true;

    #[test]
    fn empty_board() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = RenderStdout::new();
        assert_eq!(r.characters_wide(), 0);

        if TEST_HEIGHT {
            assert_eq!(r.characters_high(), 0);
        }
    }

    #[test]
    fn single_placed_tile_at_origin() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![Axial::zero()]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH);

        if TEST_HEIGHT {
            assert_eq!(r.characters_high(), TEMPLATE_HEIGHT);
        }
    }

    #[test]
    fn single_placed_tile_not_at_origin() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/ xxx  \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![Axial::new(1, 0)]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH);
        if TEST_HEIGHT {
            assert_eq!(r.characters_high(), TEMPLATE_HEIGHT);
        }
    }

    #[test]
    fn three_tiles_u_shape_1() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/ xxx  \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/ xxx  \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![
            Axial::new(0, 0),
            Axial::new(1, 0),
            Axial::new(2, -1),
        ]);
        assert_eq!(
            r.characters_wide(),
            TEMPLATE_WIDTH + (2 * TEMPLATE_WIDTH_ADD)
        );
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_HORIZ_ADD
            );
        }
    }

    #[test]
    fn three_tiles_u_shape_2() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/ xxx  \____/ xxx  \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/ xxx  \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![
            Axial::new(1, 0),
            Axial::new(2, 0),
            Axial::new(3, -1),
        ]);
        assert_eq!(
            r.characters_wide(),
            TEMPLATE_WIDTH + (2 * TEMPLATE_WIDTH_ADD)
        );
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_HORIZ_ADD
            );
        }
    }

    #[test]
    fn two_tiles_adjacent_vertically() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![Axial::new(0, 0), Axial::new(0, 1)]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH);
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_VERT_ADD
            );
        }
    }

    #[test]
    fn three_tiles_adjacent_vertically() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![
            Axial::new(0, 0),
            Axial::new(0, 1),
            Axial::new(0, 2),
        ]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH);
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_VERT_ADD + TEMPLATE_HEIGHT_VERT_ADD
            );
        }
    }

    #[test]
    fn four_tiles_adjacent_vertically() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![
            Axial::new(0, 0),
            Axial::new(0, 1),
            Axial::new(0, 2),
            Axial::new(0, 3),
        ]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH);
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT
                    + TEMPLATE_HEIGHT_VERT_ADD
                    + TEMPLATE_HEIGHT_VERT_ADD
                    + TEMPLATE_HEIGHT_VERT_ADD
            );
        }
    }

    #[test]
    fn two_tiles_diagonal_1() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/ xxx  \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */
        let r = render_with_tiles_at_positions(vec![Axial::new(0, 0), Axial::new(3, 0)]);
        assert_eq!(
            r.characters_wide(),
            TEMPLATE_WIDTH + (3 * TEMPLATE_WIDTH_ADD)
        );
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + (3 * TEMPLATE_HEIGHT_HORIZ_ADD)
            );
        }
    }

    #[test]
    fn two_tiles_diagonal_2() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/ xxx  \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/ xxx  \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![Axial::new(1, 0), Axial::new(4, 0)]);
        assert_eq!(
            r.characters_wide(),
            TEMPLATE_WIDTH + (3 * TEMPLATE_WIDTH_ADD)
        );
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + (3 * TEMPLATE_HEIGHT_HORIZ_ADD)
            );
        }
    }

    #[test]
    fn two_tiles_shallow_diagonal_1() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/ xxx  \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */
        let r = render_with_tiles_at_positions(vec![Axial::new(0, 0), Axial::new(3, -1)]);
        assert_eq!(
            r.characters_wide(),
            TEMPLATE_WIDTH + (3 * TEMPLATE_WIDTH_ADD)
        );
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_HORIZ_ADD
            );
        }
    }

    #[test]
    fn two_tiles_diagonally_adjacent_1() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/ xxx  \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![Axial::new(0, 0), Axial::new(1, 0)]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH + TEMPLATE_WIDTH_ADD);
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_HORIZ_ADD
            );
        }
    }

    #[test]
    fn two_tiles_diagonally_adjacent_backwards() {
        let r = render_with_tiles_at_positions(vec![Axial::zero(), Axial::zero().south_west()]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH + TEMPLATE_WIDTH_ADD);
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_HORIZ_ADD
            );
        }
    }

    #[test]
    fn two_tiles_diagonally_adjacent_2() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/ xxx  \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/ xxx  \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![Axial::new(1, 0), Axial::new(2, 0)]);
        assert_eq!(r.characters_wide(), TEMPLATE_WIDTH + TEMPLATE_WIDTH_ADD);
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + TEMPLATE_HEIGHT_HORIZ_ADD
            );
        }
    }

    #[test]
    fn two_tiles_diagonally_adjacent_3() {
        /*
             ____        ____        ____        ____
            /    \      /    \      /    \      /    \
           /      \____/ xxx  \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           / xxx  \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
            /    \      /    \      /    \      /    \
           /      \____/      \____/      \____/      \
           \      /    \      /    \      /    \      /
            \____/      \____/      \____/      \____/
        */

        let r = render_with_tiles_at_positions(vec![Axial::new(0, 2), Axial::new(2, -1)]);
        assert_eq!(
            r.characters_wide(),
            TEMPLATE_WIDTH + (2 * TEMPLATE_WIDTH_ADD)
        );
        if TEST_HEIGHT {
            assert_eq!(
                r.characters_high(),
                TEMPLATE_HEIGHT + (2 * TEMPLATE_HEIGHT_VERT_ADD)
            );
        }
    }
}
