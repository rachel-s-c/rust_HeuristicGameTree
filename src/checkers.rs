use crate::general_game::print_piece;
use crate::general_game::{Piece};
use std::cmp::max;
use std::io::{stdin, stdout, Write};

#[derive(Clone)]
struct CheckersGame {
    board: [Option<Piece>; 32],
    winner: Option<Piece>,
}

impl<'a> CheckersGame {
    pub fn new() -> Self {
        CheckersGame {
            board: [
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                Some(Piece::X),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
                Some(Piece::O),
            ],
            winner: None,
        }
    }

    pub fn print_board(&self) {
        println!("  A B C D E F G H");
        for i in 0..4 {
            println!(
                "{}   {}   {}   {}   {}",
                i * 2 + 1,
                print_piece(self.board[0 + i * 8]),
                print_piece(self.board[1 + i * 8]),
                print_piece(self.board[2 + i * 8]),
                print_piece(self.board[3 + i * 8]),
            );
            println!(
                "{} {}   {}   {}   {}   ",
                i * 2 + 2,
                print_piece(self.board[4 + i * 8]),
                print_piece(self.board[5 + i * 8]),
                print_piece(self.board[6 + i * 8]),
                print_piece(self.board[7 + i * 8]),
            );
        }
    }

    pub fn valid_move(&self, start: usize, end: usize) -> bool {
        false
    }
}

/*
0     1     2     3
   4     5     6     7
8     9     10    11
   12    13    14    15
16    17    18    19
   20    21    22    23
24    25    26    27
   28    29    30    31
*/

pub fn start_checkers(_difficulty: usize) {
    let game = CheckersGame::new();
    game.print_board();
    println!("Enter start location and end location as such:  6B 5A");
    while game.winner == None {
        print!(">>> ");
        let _ = stdout().flush();
        let mut loc = String::new();
        stdin()
            .read_line(&mut loc)
            .expect("Did not enter a correct string");
        let mut loc = loc.chars();
        let mut start = 0;
        let mut end = 0;
        if let Some(c) = loc.next() {
            let mut c = c as u8;
            if c > 96 {
                c = c - 32;
            }
            if c < 73 && c > 64 {
                start += (c - 65 - (c - 1) % 2) / 2;
            } else {
                println!("Incorrect starting position column");
            }
        }
        if let Some(c) = loc.next() {
            let c = c as u8;
            if c < 57 && c > 48 {
                let c = c - 49;
                start += c * 4;
            } else {
                println!("Incorrect starting position row");
            }
        }
        println!("{}", start);
        // O goes first
    }
}

#[cfg(test)]
mod check_tests {
    use super::print_piece;
    use super::CheckersGame;
    use super::Piece;

    #[test]
    fn new_check_test() {
        let check_1 = CheckersGame::new();
        assert_eq!(check_1.winner, None);
    }
}
