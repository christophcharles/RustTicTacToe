
use super::*;
use std::io;
use std::io::Write;

pub struct HumanPlayer {
    symbol: TicTacToeSymbol,
}

impl HumanPlayer {
    pub fn new(symbol: TicTacToeSymbol) -> HumanPlayer {
        HumanPlayer {
            symbol: symbol,
        }
    }
}

impl Player<TicTacToeState, TicTacToeMove, TicTacToeSymbol> for HumanPlayer {
    fn get_symbol(&self) -> TicTacToeSymbol {
        self.symbol
    }
    fn get_next_move(&self, game_state: &TicTacToeState) -> TicTacToeMove {
        loop {
            let potential_move = ask_player_move();
            let reformated_move = TicTacToeMove(potential_move.0 as u32, potential_move.1 as u32);

            if game_logic::is_move_valid(&game_state, &reformated_move) {
                return reformated_move;
            } else {
                println!("Invalid move. Please try something else.");
            }
        }
    }
}

fn read_number_from_user() -> Option<i32> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let potential_number = user_input.trim().parse::<i32>();

    if let Ok(number) = potential_number {
        Some(number)
    } else {
        None
    }
}

fn ask_number_within_bounds(message: &str, bounds: (i32,i32)) -> i32 {
    let (min, max) = bounds;

    loop {
        print!("{}", message);
        io::stdout().flush().unwrap();

        let num = read_number_from_user();

        if let Some(number) = num {
            if number < min || number > max {
                println!("This number is not within bounds which are {}..{}. Please try again again.", min, max);
            } else {
                return number;
            }
        } else {
            println!("This is not a number. Please try again again.");
        }
    }
}

fn ask_player_move() -> (i32,i32) {
    let line = ask_number_within_bounds("Please enter the line you want play in: ", (1,3));
    let column = ask_number_within_bounds("Please enter the column you want play in: ", (1,3));

    (column-1,line-1)
}
