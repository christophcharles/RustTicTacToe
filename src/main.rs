use tic_tac_toe::*;
use std::io;

fn get_letter_from_user() -> Option<char> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let potential_letter = user_input.trim().parse::<char>();

    if let Ok(letter) = potential_letter {
        Some(letter)
    } else {
        None
    }
}

fn init_player(symbol: TicTacToeSymbol)
    -> Box<dyn Player<TicTacToeState, TicTacToeMove,TicTacToeSymbol>> {
    loop {
        println!("Configuring player for {}.",
            if symbol == TicTacToeSymbol::Cross { "Crosses" } else {"Circles "});

        println!("Select the type of player by typing one of the following single letters:");
        println!(" - enter 'h' for a human player;");
        println!(" - enter 's' for a simple computer player;");
        println!(" - enter 'r' for a random computer player;");
        println!(" - enter 'c' for a more robust computer player.");

        match get_letter_from_user() {
            Some('h') => return Box::new(human_player::HumanPlayer::new(symbol)),
            Some('s') => return Box::new(computer_player::SimpleComputerPlayer::new(symbol)),
            Some('r') => return Box::new(computer_player::RandomComputerPlayer::new(symbol)),
            Some('c') => println!("Robust computer player not supported yet. Try again."),
            _ => println!("Invalid entry. Try again.")
        }
    }
}

fn main() {
    let mut state = TicTacToeState::new();

    println!("Crosses will start.");

    let player1 = init_player(TicTacToeSymbol::Cross);
    let player2 = init_player(TicTacToeSymbol::Circle);

    let mut cross_turn = true;
    loop {
        game_logic::draw_board(&state);

        if cross_turn {
            println!("Crosses are playing.");
        } else {
            println!("Circles are playing.");
        }
        let current_player = if cross_turn {&player1} else {&player2};
        let cell = current_player.get_next_move(&state);

        if game_logic::play_move(&mut state, &cell, current_player.get_symbol()) {
            if game_logic::has_someone_won(&state) || game_logic::is_board_full(&state) {
                break;
            }

            cross_turn = !cross_turn;
        } else {
            println!("Invalid cell! Try another cell.");
        }
    }

    game_logic::draw_board(&state);

    if game_logic::has_someone_won(&state) {
        if cross_turn {
            println!("Crosses have won!");
        } else {
            println!("Circles have won!");
        }
    } else {
        println!("It is a draw.")
    }
}
