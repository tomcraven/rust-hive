#![allow(dead_code)]
#![allow(unused_imports)]

extern crate colored;

mod player;
mod tile;
mod game;
mod position;
mod board;
mod output;
mod bounds;

use game::Game;
use player::*;
use tile::standard_game_tiles;

fn main() {
    let mut g = Game::new(Player::new(standard_game_tiles()),
                          Player::new(standard_game_tiles()));
    g.go();
}
