#![allow(dead_code, unused_imports, unused_variables, unused_macros)]

extern crate colored;

mod axial;
mod board;
mod bounds;
mod character_buffer_2d;
mod coord_utils;
mod game;
mod placed_tile;
mod player;
mod position;
mod render;
mod tile;

use game::Game;
use player::*;
use tile::standard_game_tiles;

fn main() {
    let mut g = Game::new(
        Player::new(standard_game_tiles(), PlayerNumber::One),
        Player::new(standard_game_tiles(), PlayerNumber::Two),
    );
    g.go();
}
