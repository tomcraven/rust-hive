use axial::Axial;
use board::*;
use player::PlayerNumber;
use player::*;
use std::rc::Rc;
use tile::*;

pub struct Game {
    player1: Player,
    player2: Player,
    board: Board,
}

impl Game {
    pub fn new(p1: Player, p2: Player) -> Game {
        Game {
            player1: p1,
            player2: p2,
            board: Board::new(),
        }
    }

    pub fn go(&mut self) {
        self.board
            .place_tile(queen(), Axial::zero(), PlayerNumber::One);
        self.board
            .place_tile(beetle(), Axial::zero().south(), PlayerNumber::One);
        self.board
            .place_tile(spider(), Axial::zero().south().south(), PlayerNumber::One);
        self.board.place_tile(
            ant(),
            Axial::zero().south().south().south(),
            PlayerNumber::One,
        );
        self.board.place_tile(
            ant(),
            Axial::zero().south().south().south().south_west(),
            PlayerNumber::One,
        );
        self.board.place_tile(
            ant(),
            Axial::zero().south().south().south().north_west(),
            PlayerNumber::One,
        );
        self.board.place_tile(
            ant(),
            Axial::zero().south().south().south().south_east(),
            PlayerNumber::One,
        );

        self.board.place_tile(
            grass_hopper(),
            Axial::zero().south().north_east(),
            PlayerNumber::One,
        );

        self.board.render();
    }
}
