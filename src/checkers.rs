use super::*;
use std::io::{stdin, stdout, Write};
use crate::minimax;

// Could be adjusted by user, but must be even for our algos
const BOARDWIDTH: usize = 8;
const BOARDSIZE: usize = BOARDWIDTH * BOARDWIDTH / 2;

#[derive(Clone)]
struct CheckersGame {
    board: [Option<Piece>; BOARDSIZE],
    o_won: Option<bool>,
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
                Some(Piece::X),Some(Piece::X),Some(Piece::X),Some(Piece::X),
                Some(Piece::X),Some(Piece::X),Some(Piece::X),Some(Piece::X),
                Some(Piece::X),Some(Piece::X),Some(Piece::X),Some(Piece::X),
                None,None,None,None,
                None,None,None,None,
                Some(Piece::O),Some(Piece::O),Some(Piece::O),Some(Piece::O),
                Some(Piece::O),Some(Piece::O),Some(Piece::O),Some(Piece::O),
                Some(Piece::O),Some(Piece::O),Some(Piece::O),Some(Piece::O),
            ],
            o_won: None,
            is_o_turn: true,
            last_skip: None,
        }
    }

    pub fn print_board(&self) {
        println!("  A B C D E F G H");
        for i in 0..4 {
            println!(
                "{} {}   {}   {}   {}",
                i * 2 + 1,
                print_piece(&self.board[0 + i * 8]),
                print_piece(&self.board[1 + i * 8]),
                print_piece(&self.board[2 + i * 8]),
                print_piece(&self.board[3 + i * 8]),
            );
            println!(
                "{}   {}   {}   {}   {}   ",
                i * 2 + 2,
                print_piece(&self.board[4 + i * 8]),
                print_piece(&self.board[5 + i * 8]),
                print_piece(&self.board[6 + i * 8]),
                print_piece(&self.board[7 + i * 8]),
            );
        }
    }
    pub fn possible_positions_jump(&self, start: usize) -> Vec<(usize,usize,Option<usize>)> {
		let mut pos: Vec<(usize,usize,Option<usize>)> = Vec::new();
		// Abstracted out to work with any size board
		let threshold = BOARDWIDTH / 2 -1;
		if (start + BOARDWIDTH - 1) % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				// cannot combine if statements, else risk out of bounds
				if let Some(p) = &self.board[start + BOARDWIDTH / 2 - 1] {
					if p.is_x() == self.is_o_turn {
						// Found a bordering enemy, add it to pos vec
						pos.push((start,start + BOARDWIDTH - 1, Some(start + BOARDWIDTH / 2 - 1)));
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2 - 1] {
					if p.is_x() == self.is_o_turn {
						pos.push((start,start - BOARDWIDTH - 1, Some(start - BOARDWIDTH / 2 - 1)));
					}
				}
			}
		}
		if (start + BOARDWIDTH / 2 - 1) % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				if let Some(p) = &self.board[start + BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						pos.push((start,start + BOARDWIDTH - 1, Some(start + BOARDWIDTH / 2)));
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						pos.push((start,start - BOARDWIDTH - 1, Some(start - BOARDWIDTH / 2)));
					}
				}
			}
		}
		if start % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				if let Some(p) = &self.board[start + BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						pos.push((start,start + BOARDWIDTH + 1, Some(start + BOARDWIDTH / 2)));
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2] {
					if p.is_x() == self.is_o_turn {
						pos.push((start,start - BOARDWIDTH + 1, Some(start - BOARDWIDTH / 2)));
					}
				}
			}
		}
		if (start + BOARDWIDTH / 2) % BOARDWIDTH < threshold {
			if start < BOARDSIZE - BOARDWIDTH {
				if let Some(p) = &self.board[start + BOARDWIDTH / 2 + 1] {
					if p.is_x() == self.is_o_turn {
						pos.push((start,start + BOARDWIDTH + 1, Some(start + BOARDWIDTH / 2 + 1)));
					}
				}
			}
			if start >= BOARDWIDTH {
				if let Some(p) = &self.board[start - BOARDWIDTH / 2 + 1] {
					if p.is_x() == self.is_o_turn {
						pos.push((start,start - BOARDWIDTH + 1, Some(start - BOARDWIDTH / 2 + 1)));
					}
				}
			}
		}
		// Filter out any skips where the destination is not empty
		pos.into_iter()
			.filter(move |p| {
				self.board[p.1].is_none() && {
					if self.board[p.0].as_ref().unwrap().is_king() {
						true
					}
					else if self.board[p.0].as_ref().unwrap().is_x() {
						p.0 < p.1
					}
					else {
						p.1 < p.0
					}
				}
			})
			.collect()
    }
    pub fn possible_positions_no_jump(&self, start: usize) -> Vec<(usize,usize,Option<usize>)> {
    	// In American checkers, if a jump is possible from a player, the player must make the jump
    	// thus it will be the only possible move
		let p = &self.board[start].clone().expect("Cannot check possible positions from a position with no Piece");
		let mut pos: Vec<(usize,usize,Option<usize>)> = Vec::new();

		if (start + 7) % 8 < 3 {
			if p.is_x() || p.is_king() {
				pos.push((start, start + 3, None));
			}
			if (p.is_o() || p.is_king()) && start > BOARDWIDTH {
				pos.push((start, start - 5, None));
			}
		}
		if (start + 4) % 8 < 3 {
			if (p.is_x() || p.is_king()) && start < BOARDSIZE - BOARDWIDTH/2 {
				pos.push((start, start + 5, None));
			}
			if p.is_o() || p.is_king() {
				pos.push((start, start - 3, None));
			}
		}
		if start < BOARDSIZE - BOARDWIDTH/2 {
			if p.is_x() || p.is_king() {
				pos.push((start, start + 4, None));
			}
		}
		if start >= BOARDWIDTH/2 {
			if p.is_o() || p.is_king() {
				pos.push((start, start - 4, None));
			}
		}
		pos.into_iter().filter(move |p| self.board[p.1].is_none()).collect()
    }
    pub fn valid_move(&self, start: usize, end: usize) -> Option<(usize,usize,Option<usize>)> {
    	self.possible_moves().find(|m| m.0 == start && m.1 == end)
    }
    pub fn maybe_make_king(&mut self, pos: usize) {
    	if pos < BOARDWIDTH/2 && self.board[pos].as_ref().unwrap().is_o() {
			self.board[pos] = Some(Piece::OKing);
    	}
    	else if pos >= BOARDSIZE - BOARDWIDTH/2 && self.board[pos].as_ref().unwrap().is_x() {
    		self.board[pos] = Some(Piece::XKing);
    	}
    }
    pub fn is_o_winner(&self) -> Option<bool> {
    	let mut found_o = false;
    	let mut found_x = false;
    	for i in 0..BOARDSIZE { // Search until we find an x and o piece
    		if let Some(p) = &self.board[i] {
    			found_x |= p.is_x();
    			found_o |= p.is_o();
    		}
    		if found_x && found_o {
    			return None;
    		}
    	}
    	Some(found_o)
    }
    pub fn check_winner(&mut self) {
    	self.o_won = self.is_o_winner();
    }
    pub fn who_won(&self) -> Option<&str> {
    	if let Some(true) = self.o_won {
    		Some("O")
    	}
    	else if let Some(false) = self.o_won {
    		Some("X")
    	}
    	else {
    		None
    	}
    }
    pub fn is_opponent_turn(&self) -> bool {
    	!self.is_o_turn
    }
    pub fn give_up_turn(&mut self) {
    	self.is_o_turn = !self.is_o_turn;
    }
}

impl<'a> HeuristicGameTree for CheckersGame {
	type Move = (usize,usize,Option<usize>);

    fn heuristic(&self) -> isize {
    	let mut h: isize = 0;
    	for i in 0..BOARDSIZE {
    		if let Some(p) = &self.board[i] {
    			if p.is_o() {
    				h -= 1;
    			}
    			else {
    				h += 1;
    			}
    		}
    	}
    	h
    }
    fn execute_move(&mut self, m: &Self::Move, _is_opponent: bool) -> bool {
    	self.last_skip = None;
    	if let Some(jumped) = m.2 {
    		self.board[jumped] = None;
    		self.board[m.1] = self.board[m.0].clone();
    		self.board[m.0] = None;
    		self.maybe_make_king(m.1);
    		if self.possible_positions_jump(m.1).len() == 0 {
    			// No jumps left from this piece, give over turn
    			self.is_o_turn = !self.is_o_turn;
    		}
    		else {
    			// Another jump available, assign so that it only
    			// gives possible moves of this jump
    			self.last_skip = Some(m.1);
    		}
    	}
    	else {
    		self.board[m.1] = self.board[m.0].clone();
    		self.board[m.0] = None;
    		self.maybe_make_king(m.1);
    		self.is_o_turn = !self.is_o_turn; // Next player's move
    	}
    	!self.is_o_turn
    }
	fn possible_moves(&self) -> Box<Iterator<Item=(usize,usize,Option<usize>)> + '_> {
    	let mut positions: Vec<(usize,usize,Option<usize>)> = Vec::new();
    	if let Some(p) = self.last_skip {
    		positions.extend(&self.possible_positions_jump(p));
    	}
    	else {
	    	for (i, start) in self.board.as_ref().iter().enumerate() {
	    		if let Some(s) = start {
	    			if self.is_o_turn == s.is_o() {
	    				positions.extend(&self.possible_positions_jump(i));
	    			}
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
    	Box::new(positions.into_iter())
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
/// Starts the Checkers game
///
/// # Arguments
///
/// * `difficulty` - A usize that holds the difficulty of the game. This value is passed as
///                  an argument to minimax search, determining the depth of the minimax search
///                  tree (i.e. the number of steps ahead that the AI agent should look ahead when
///                  determining its move)
pub fn start_checkers(difficulty: usize) {
    let mut game = CheckersGame::new();
    println!("Enter start location and end location as such:  B6 A5. You are o");
    while game.who_won().is_none() {
    	if game.is_opponent_turn() {
    		game.print_board();
    	}
    	while game.is_opponent_turn() {
    		print!("Opponent's move ... ");
    		let next_move = minimax::minimax_search(&game, difficulty * 12, true);
    		println!("Done");
    		if let Some(m) = next_move {
    			game.execute_move(&m,true);
    		}
    		else {
    			println!("Should never reach here");
    			game.give_up_turn();
    		}
			game.check_winner();
    	}
    	if game.who_won().is_some() {
    		break;
    	}
    	game.print_board();
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
                println!("Incorrect starting position column, please enter again");
                continue;
            }
        }
        if let Some(c) = loc.next() {
            let c = c as u8;
            if c < 57 && c > 48 {
                let c = c - 49;
                start += c * 4;
            } else {
                println!("Incorrect starting position row, please enter again");
                continue;
            }
        }
        let _ = loc.next();
        if let Some(c) = loc.next() {
            let mut c = c as u8;
            if c > 96 {
                c = c - 32;
            }
            if c < 73 && c > 64 {
                end += (c - 65 - (c - 1) % 2) / 2;
            } else {
                println!("Incorrect starting position column, please enter again");
                continue;
            }
        }
        if let Some(c) = loc.next() {
            let c = c as u8;
            if c < 57 && c > 48 {
                let c = c - 49;
                end += c * 4;
            } else {
                println!("Incorrect starting position row, please enter again");
                continue;
            }
        }
		if let Some(mymove) = game.valid_move(start as usize, end as usize) {
			game.execute_move(&mymove,false);
			game.check_winner();
		}
		else {
			println!("Invalid move (remember if you have a jump, you must take it");
		}
    }
    game.print_board();
    println!("The winning piece is {}",game.who_won().unwrap());
}

// #[cfg(test)]
// mod check_tests {
//     use super::print_piece;
//     use super::CheckersGame;
//     use super::Piece;

//     #[test]
//     fn new_check_test() {
//         let check_1 = CheckersGame::new();
//         assert!(check_1.winner.is_none());
//     }
// }
