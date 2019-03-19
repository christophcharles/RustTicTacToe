
use super::*;

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
        return TicTacToeMove(0,0);
    }
}
