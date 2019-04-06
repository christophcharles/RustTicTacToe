pub mod game_logic;
pub mod human_player;
pub mod computer_player;

pub trait Player<S,M,P> {
    fn get_symbol(&self) -> P;
    fn get_next_move(&self, game_state: &S) -> M;
}

#[derive(Clone, Copy, PartialEq)]
pub enum TicTacToeSymbol {
    Cross,
    Circle,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BoardCell {
    Empty,
    Played(TicTacToeSymbol),
}

#[derive(Clone, Copy)]
pub struct TicTacToeState {
    pub board: [BoardCell; 9],
}

impl TicTacToeState {
    pub fn new() -> TicTacToeState {
        TicTacToeState {
            board: [BoardCell::Empty; 9],
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct TicTacToeMove(u32,u32);
