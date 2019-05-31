use std::io::{stdin,stdout,Write};
use std::cmp::max;


#[derive (PartialEq, Copy, Clone, Debug)]
pub enum Piece {
    X,
    O,
}

impl Piece {
    fn is_x(&self) -> bool {
        if let Piece::X = self {
            true
        }
        else {false}
    }
}

pub fn print_piece<'a>(item: Option<Piece>) -> &'a str {
    if item.is_none()
    {
        " "
    }
    else if item.unwrap().is_x()
    {
        "X"
    }
    else {
    	"O"
    }
}

#[derive (Clone)]
struct CheckersGame {
    board: [Option<Piece>; 32],
    winner: Option<Piece>,
}

impl<'a> CheckersGame {
	pub fn new() -> Self {
		CheckersGame {
			board: [
				Some(Piece::O),Some(Piece::O),Some(Piece::O),Some(Piece::O),
				Some(Piece::O),Some(Piece::O),Some(Piece::O),Some(Piece::O),
				Some(Piece::O),Some(Piece::O),Some(Piece::O),Some(Piece::O),
				None,None,None,None,
				None,None,None,None,
				Some(Piece::X),Some(Piece::X),Some(Piece::X),Some(Piece::X),
				Some(Piece::X),Some(Piece::X),Some(Piece::X),Some(Piece::X),
				Some(Piece::X),Some(Piece::X),Some(Piece::X),Some(Piece::X),
			],
			winner: None,
		}
	}

	pub fn printboard(&self) {
		for i in 0..4 {
			println!("  {}   {}   {}   {}", print_piece(self.board[0 + i*8]),  print_piece(self.board[1 + i*8]), print_piece(self.board[2 + i*8]), print_piece(self.board[3 + i*8]),);
			println!("{}   {}   {}   {}   ", print_piece(self.board[4 + i*8]),  print_piece(self.board[5 + i*8]), print_piece(self.board[6 + i*8]), print_piece(self.board[7 + i*8]),);
		}
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
	game.printboard();
}