
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

pub struct MinimaxComputerPlayer {
    symbol: TicTacToeSymbol,
}

impl MinimaxComputerPlayer {
    pub fn new(symbol: TicTacToeSymbol) -> MinimaxComputerPlayer {
        MinimaxComputerPlayer {
            symbol: symbol,
        }
    }

    pub fn get_best_score_and_move_for_symbol(&self, game_state: &TicTacToeState, symbol: TicTacToeSymbol)
        -> (i32, Vec<TicTacToeMove>) {
        let mut best_moves = Vec::new();
        let mut best_score = -1;
        for i in 0..3 {
            for j in 0..3 {
                let new_move = TicTacToeMove(i as u32, j as u32);
                if game_logic::is_move_valid(game_state, &new_move) {
                    let mut new_game_state = *game_state;
                    game_logic::play_move(&mut new_game_state, &new_move, symbol);

                    let move_score = if let Some(winning_symbol) = game_logic::has_someone_won(&new_game_state) {
                        if winning_symbol == symbol { 1 } else { -1 }
                    } else if game_logic::is_board_full(&new_game_state) {
                        0
                    } else {
                        let (best_score_for_ennemy, _) = self.get_best_score_and_move_for_symbol(&new_game_state, if symbol == TicTacToeSymbol::Cross {TicTacToeSymbol::Circle } else {TicTacToeSymbol::Cross});
                        -best_score_for_ennemy
                    };

                    if move_score > best_score {
                        best_score = move_score;
                        best_moves = vec![new_move];
                    } else if move_score == best_score {
                        best_moves.push(new_move);
                    }
                }
            }
        }

        if best_moves.is_empty() {
            panic!("No valid move found");
        }

        (best_score, best_moves)
    }
}

impl Player<TicTacToeState, TicTacToeMove, TicTacToeSymbol> for MinimaxComputerPlayer {
    fn get_symbol(&self) -> TicTacToeSymbol {
        self.symbol
    }
    fn get_next_move(&self, game_state: &TicTacToeState) -> TicTacToeMove {
        let (_, best_moves) = self.get_best_score_and_move_for_symbol(game_state, self.symbol);
        let move_count = best_moves.len();
        best_moves[rand::thread_rng().gen_range(0,move_count)]
    }
}
