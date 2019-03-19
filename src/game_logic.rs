use super::*;

fn has_someone_won_check_line(state: &TicTacToeState, line: usize) -> bool {
    if state.board[line*3] == BoardCell::Empty {
        false
    } else {
        for j in 1..3 {
            if state.board[line*3+j] != state.board[line*3+j-1] {
                return false
            }
        }
        true
    }
}

fn has_someone_won_check_column(state: &TicTacToeState, col: usize) -> bool {
    if state.board[col] == BoardCell::Empty {
        false
    } else {
        for i in 1..3 {
            if state.board[i*3+col] != state.board[(i-1)*3+col] {
                return false
            }
        }
        true
    }
}

fn has_someone_won_check_diagonal(state: &TicTacToeState) -> bool {
    if state.board[0] == BoardCell::Empty {
        false
    } else {
        for i in 1..3 {
            if state.board[i*3+i] != state.board[(i-1)*3+i-1] {
                return false
            }
        }
        true
    }
}

fn has_someone_won_check_antidiagonal(state: &TicTacToeState) -> bool {
    if state.board[2] == BoardCell::Empty {
        false
    } else {
        for i in 1..3 {
            if state.board[(2-i)*3+i] != state.board[(3-i)*3+i-1] {
                return false
            }
        }
        true
    }
}

pub fn has_someone_won(state: &TicTacToeState) -> bool {
    for i in 0..3 {
        if has_someone_won_check_line(state, i) {
            return true;
        }
    }

    for j in 0..3 {
        if has_someone_won_check_column(state, j) {
            return true;
        }
    }

    has_someone_won_check_diagonal(state) || has_someone_won_check_antidiagonal(state)
}

pub fn is_board_full(state: &TicTacToeState) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if state.board[i*3+j] == BoardCell::Empty {
                return false;
            }
        }
    }
    true
}

pub fn is_move_valid(state : &TicTacToeState, cell: &TicTacToeMove) -> bool {
    let x = cell.0 as usize;
    let y = cell.1 as usize;

    if (x >= 3) || (y >= 3) {
        return false;
    }

    let stride = y*3+x;

    state.board[stride] == BoardCell::Empty
}

pub fn play_move(state : &mut TicTacToeState, cell: &TicTacToeMove, symbol: TicTacToeSymbol) -> bool {
    if !is_move_valid(&state, cell) {
        false
    } else {
        let x = cell.0 as usize;
        let y = cell.1 as usize;

        let stride = y*3+x;
        state.board[stride] = BoardCell::Played(symbol);
        true
    }
}

pub fn draw_board(state: &TicTacToeState) -> () {
    for i in 0..3 {
        print!("|");
        for j in 0..3 {
            match state.board[i*3+j] {
                BoardCell::Empty => print!(" "),
                BoardCell::Played(TicTacToeSymbol::Cross) => print!("X"),
                BoardCell::Played(TicTacToeSymbol::Circle) => print!("O")
            }
            print!("|");
        }
        print!("\n");
    }
}
