
use super::*;
use rand::Rng;

pub struct SimpleComputerPlayer {
    symbol: TicTacToeSymbol,
}

impl SimpleComputerPlayer {
    pub fn new(symbol: TicTacToeSymbol) -> SimpleComputerPlayer {
        SimpleComputerPlayer {
            symbol: symbol,
        }
    }
}

impl Player<TicTacToeState, TicTacToeMove, TicTacToeSymbol> for SimpleComputerPlayer {
    fn get_symbol(&self) -> TicTacToeSymbol {
        self.symbol
    }
    fn get_next_move(&self, game_state: &TicTacToeState) -> TicTacToeMove {
        for i in 0..3 {
            for j in 0..3 {
                let computer_move = TicTacToeMove(i as u32, j as u32);
                if game_logic::is_move_valid(game_state, &computer_move) {
                    return computer_move;
                }
            }
        }
        panic!("get_next_move called by no more cell available");
    }
}

pub struct RandomComputerPlayer {
    symbol: TicTacToeSymbol,
}

impl RandomComputerPlayer {
    pub fn new(symbol: TicTacToeSymbol) -> RandomComputerPlayer {
        RandomComputerPlayer {
            symbol: symbol,
        }
    }
}

impl Player<TicTacToeState, TicTacToeMove, TicTacToeSymbol> for RandomComputerPlayer {
    fn get_symbol(&self) -> TicTacToeSymbol {
        self.symbol
    }
    fn get_next_move(&self, game_state: &TicTacToeState) -> TicTacToeMove {
        loop {
            let i = rand::thread_rng().gen_range(0,3);
            let j = rand::thread_rng().gen_range(0,3);

            let computer_move = TicTacToeMove(i as u32, j as u32);
            if game_logic::is_move_valid(game_state, &computer_move) {
                return computer_move;
            }
        }
    }
}
