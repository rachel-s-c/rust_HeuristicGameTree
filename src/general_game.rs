use crate::checkers::start_checkers;
use crate::connect4::start_con;
use crate::tictactoe::start_tic;

/// Starts the game after the player runs the program with the selected game.
///
/// Available options are Tic-tac-toe, Connect4, and Checkers.
/// # Arguments
/// * `game` - A usize that holds the number corresponding to the game type
/// * `diff` - A usize that holds the difficulty of the game, which is either the default setting
///            (medium, corresponding to the number 2), or a difficulty setting that has been chosen
///            by the player (easy 1, medium 2, hard 3).
///
pub fn lets_play(game: usize, diff: usize) {
    match game {
        1 => start_tic(diff),
        2 => start_con(diff),
        3 => start_checkers(diff),
        _ => println!("error"),
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
/// Game pieces for a two-player game where all the pieces have equal value.
///
/// Used in games such as Tic-tac-toe, Connect4, Mancala, etc.
pub enum Piece {
    /// An 'X' piece
    X,
    /// An 'O' piece
    O,
}

impl Piece {
    fn is_x(self) -> bool {
        if let Piece::X = self {
            true
        } else {
            false
        }
    }
    fn is_o(self) -> bool {
        if let Piece::O = self {
            true
        } else {
            false
        }
    }
}

/// Prints the game piece on the game board.
///
/// # Arguments
/// * `item` - Of type Option<Piece>, where it is either a None, or it holds an 'X' or 'O' game
/// Piece. If it is a Some(X) or Some(O), then the content is unwrapped and printed.
///
/// Returns a reference to a string, which is the element that is printed on the board.
///
/// # Example
/// ```
/// # use heuristic_game_tree::general_game::print_piece;
/// # use heuristic_game_tree::general_game::Piece;
///
/// let x = Some(Piece::X);
/// let print = print_piece(x);
/// assert_eq!(print, "X")
/// ```
///
/// ```
/// # use heuristic_game_tree::general_game::print_piece;
/// let x = None;
/// let print = print_piece(x);
/// assert_eq!(print, " ")
///
/// ```
pub fn print_piece<'a>(item: Option<Piece>) -> &'a str {
    if item.is_none() {
        return " ";
    }
    if item.unwrap().is_x() {
        return "X";
    }
    "O"
}



#[cfg(test)]
mod gen_game_tests {
    use super::print_piece;
    use super::Piece;

    #[test]
    fn x_check() {
        let x = Piece::X;
        assert!(x.is_x());
    }

    #[test]
    fn o_check() {
        let o = Piece::O;
        assert!(o.is_o());
    }

    #[test]
    fn x_printcheck() {
        let x = Piece::X;
        assert_eq!(print_piece(Some(x)), "X");
    }

    #[test]
    fn o_printcheck() {
        let o = Piece::O;
        assert_eq!(print_piece(Some(o)), "O");
    }

}
