use crate::checkers::start_checkers;
use crate::connect4::start_con;
use crate::tictactoe::start_tic;

pub fn lets_play(game: usize, diff: usize) {
    match game {
        1 => start_tic(diff),
        2 => start_con(diff),
        3 => start_checkers(diff),
        _ => println!("error"),
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Piece {
    X,
    O,
}

impl Piece {
    fn is_x(&self) -> bool {
        if let Piece::X = self {
            true
        } else {
            false
        }
    }
    fn is_o(&self) -> bool {
        if let Piece::O = self {
            true
        } else {
            false
        }
    }
}

pub fn print_piece<'a>(item: Option<Piece>) -> &'a str {
    if item.is_none() {
        return " ";
    }
    if item.unwrap().is_x() {
        return "X";
    }
    return "O";
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
