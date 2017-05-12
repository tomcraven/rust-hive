use player::*;
use board::*;

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
        let (tile, position) = self.player1.get_tile_placement(BoardProxy::new(&mut self.board));

        self.board.place_tile(tile, position, PlayerNumber::One);

        self.board.render();
    }
}
