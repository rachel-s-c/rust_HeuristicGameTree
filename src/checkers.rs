use super::*;
use std::cmp::max;
use std::io::{stdin, stdout, Write};

// Could be adjusted by user, but must be even for our algos
const BOARDWIDTH: usize = 8;
const BOARDSIZE: usize = BOARDWIDTH * BOARDWIDTH / 2;

#[derive(Clone)]
struct CheckersGame {
    board: [Option<Piece>; 32],
    winner: Option<Piece>,
    is_o_turn: bool,
    last_skip: Option<usize>,
}
#[derive(Clone)]
pub enum Piece {
    X,
    O,
    XKing,
    OKing,
}

impl Piece {
	pub fn is_o(&self) -> bool {
		match &self {
			Piece::O => true,
			Piece::OKing => true,
			_ => false
		}
	}
	pub fn is_x(&self) -> bool {
		!&self.is_o()
	}
	pub fn is_king(&self) -> bool {
		match &self {
			Piece::OKing => true,
			Piece::XKing => true,
			_ => false
		}
	}
}

pub fn print_piece<'a>(item: &Option<Piece>) -> &'a str {
    match item {
        None => " ",
        Some(p) => match p {
            Piece::X => "x",
            Piece::O => "o",
            Piece::XKing => "X",
            Piece::OKing => "O",
        },
    }
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
            is_o_turn: true,
            last_skip: None,
        }
    }

    pub fn print_board(&self) {
        println!("  A B C D E F G H");
        for i in 0..4 {
            println!(
                "{}   {}   {}   {}   {}",
                i * 2 + 1,
                print_piece(&self.board[0 + i * 8]),
                print_piece(&self.board[1 + i * 8]),
                print_piece(&self.board[2 + i * 8]),
                print_piece(&self.board[3 + i * 8]),
            );
            println!(
                "{} {}   {}   {}   {}   ",
                i * 2 + 2,
                print_piece(&self.board[4 + i * 8]),
                print_piece(&self.board[5 + i * 8]),
                print_piece(&self.board[6 + i * 8]),
                print_piece(&self.board[7 + i * 8]),
            );
        }
    }
    pub fn possible_moves(&self) -> impl Iterator<Item=(usize,usize)> + '_ {
    	let mut positions: Vec<(usize,usize)> = Vec::new();
    	for (i, start) in self.board.as_ref().iter().enumerate() {
    		if let Some(s) = start {
    			if self.is_o_turn == s.is_o() {
    				positions.extend(&self.possible_positions_jump(i));
    			}
    		}
    	}
    	// Only look for nonjumps if no jumps are available
    	// Also only look for nonjumps if we didn't just jump
    	if positions.len() == 0 && self.last_skip.is_none() {
	    	for (i, start) in self.board.as_ref().iter().enumerate() {
	    		if let Some(s) = start {
	    			if self.is_o_turn == s.is_o() {
	    				positions.extend(&self.possible_positions_no_jump(i));
	    			}
	    		}
	    	}
    	}
    	positions.into_iter()
    }
    pub fn valid_end(&self, position: usize, o_player: bool) -> bool {
    	false
    }
    pub fn possible_positions_no_jump(&self, start: usize) -> Vec<(usize,usize)> {
    	// In American checkers, if a jump is possible from a player, the player must make the jump
    	// thus it will be the only possible move
		let piece = &self.board[start].clone().expect("Cannot check possible positions from a position with no Piece");
		let mut pos: Vec<(usize,usize)> = Vec::new();
		if start == 0 { // Would be either x or O
			// Only one possible move
			pos.push((start,4));
		}
		else if start == 31 { // Would be either o or X a this point
			pos.push((start,27));
		}
		else if start % 8 == 0 || start % 8 == 7 {
			if piece.is_o() || piece.is_king() {
				pos.push((start,start-4));
			}
			if piece.is_x() || piece.is_king() {
				pos.push((start,start+4));
			}
		}
		else if start < 4 { // Would either be x or O
			pos.push((start,start+3));
			pos.push((start,start+4));
		}
		else if start > 27 { // would either be x or X
			pos.push((start,start-3));
			pos.push((start,start-4));
		}
		// Finished checking edges
		else {
			if piece.is_o() || piece.is_king() {
				pos.push((start,start-3));
				pos.push((start,start-4));
			}
			if piece.is_x() || piece.is_king() {
				pos.push((start,start+3));
				pos.push((start,start+4));
			}
		}
		pos.into_iter().filter(move |p| self.board[p.1].is_none()).collect()
    }
    pub fn possible_positions_jump(&self, start: usize) -> Vec<(usize,usize)> {
		let mut pos: Vec<(usize,usize)> = Vec::new();
		// Abstracted out to work with any size board
		let threshold = BOARDWIDTH / 2 -1;
		if (start + BOARDWIDTH - 1) % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				// cannot combine if statements, else risk out of bounds
				if let Some(p) = &self.board[start + BOARDWIDTH / 2 -1] {
					if p.is_x() == self.is_o_turn {
						// Found a skip, add it to pos vec
						let next_pos = start + BOARDWIDTH - 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2 -1] {
					if p.is_x() == self.is_o_turn {
						let next_pos = start - BOARDWIDTH - 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
		}
		if (start + BOARDWIDTH / 2 - 1) % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				if let Some(p) = &self.board[start + BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						let next_pos = start + BOARDWIDTH - 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						let next_pos = start - BOARDWIDTH - 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
		}
		if start % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				if let Some(p) = &self.board[start + BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						let next_pos = start + BOARDWIDTH + 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						let next_pos = start - BOARDWIDTH + 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
		}
		if (start + BOARDWIDTH / 2) % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				if let Some(p) = &self.board[start + BOARDWIDTH / 2 + 1] {
					if p.is_x() == self.is_o_turn {
						let next_pos = start + BOARDWIDTH + 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2 + 1] {
					if p.is_x() == self.is_o_turn {
						let next_pos = start - BOARDWIDTH + 1;
						if self.board[next_pos].is_none() {
							pos.push((start,next_pos));
						}
					}
				}
			}
		}
		pos
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
    while game.winner.is_none() {
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
        let start = start as usize;
        let end = end as usize;
        while game.winner.is_none() {
        	// O goes first, person playing is O, computer is X
        }
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
        assert!(check_1.winner.is_none());
    }
}
