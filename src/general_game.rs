use core::isize::{MAX,MIN};
use crate::connect4::start_con;
use crate::tictactoe::start_tic;
use crate::checkers::start_checkers;


pub fn lets_play(game: usize, diff: usize)
{
    match game
        {
            1 => start_tic(diff),
            2 => start_con(diff),
            3 => start_checkers(diff),
            _ => println!("error"),
        }
}
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
    fn is_o(&self) -> bool {
        if let Piece::O = self {
            true
        }
        else {false}
    }
}

pub fn print_piece<'a>(item: Option<Piece>) -> &'a str
{
    if item.is_none()
    {
       return " "
    }
    if item.unwrap().is_x()
    {
      return  "X"
    }
   return "O"

}

#[allow(unused)]
pub trait HeuristicGameTree: Clone {
    type Move: Clone + Sized;
    //type Heuristic: PartialOrd;
    // Should be a type Heuristic that can be any type that can be compared; doing isize for now

    // fn possible_moves(&self) -> Iterator<Item = Self::Move>;
    fn possible_moves(&self) -> Vec<Self::Move>;
    fn heuristic(&self) -> isize;
    fn execute_move(&mut self, next_move: &Self::Move, is_opponent: bool);

    fn minimax_search(&mut self, depth: usize, is_opponent: bool) -> Option<Self::Move> {
        // Gets the possible moves (i.e. children)
        // Makes executes each move on a copy of the game
        // Gets the heuristic of each game copy executed on a next move
        // by calling minimax again
        // Returns the move corresponding with the best heuristic
        let mut best_move = (None,MIN); // We're going to maximize heuristic
        if depth > 0 {
            for mymove in self.possible_moves() {
                let mut next_state = self.clone();
                next_state.execute_move(&mymove.clone(), !is_opponent); // Need to clone, standard procedure with minimax
                let h = next_state.minimax_helper(depth-1, false, MAX, MIN);
                if h > best_move.1 {
                    best_move = (Some(mymove),h);
                }
            }
        }
        else {
            // Choose first available move.
            let moves = self.possible_moves();
            if moves.len() == 0 {
                return None;
            }
            return Some(moves[0].clone());
        }

        best_move.0 // Return the move that corresponds with best heuristic
    }

    // Need a helper cause the client shouldn't provide alpha and beta
    // also nice because we don't have to have return valus of structs/tuples, can just do an isize
    // that corresponds to the best value for the immediately next move
    fn minimax_helper(&mut self, depth: usize, is_opponent: bool, mut alpha: isize, mut beta: isize) -> isize {
        let mut current_heuristic = self.heuristic();
        if depth > 0 { // End of depth, return
            let mut child_heuristic = if is_opponent {MIN} else {MAX};
            for mymove in self.possible_moves() {
                let mut next_state = self.clone();
                next_state.execute_move(&mymove.clone(), !is_opponent);
                let h = next_state.minimax_helper(depth-1, false, alpha, beta);
                if (h > child_heuristic && is_opponent) || (h < child_heuristic && !is_opponent) {
                    child_heuristic = h;
                }
                if is_opponent && h > alpha {
                    alpha = h;
                }
                else if !is_opponent && h > beta {
                    beta = h;
                }
                if beta < alpha {break;}
            }
            if (child_heuristic > MIN && is_opponent) || (child_heuristic < MAX && !is_opponent) {
                current_heuristic = child_heuristic;
            }
        }
        current_heuristic
    }
}

#[cfg(test)]
mod gen_game_tests {
    use super::Piece;
    use super::print_piece;

    #[test]
    fn x_check()
    {
        let x = Piece::X;
        assert!(x.is_x());
    }

    #[test]
    fn o_check()
    {
        let o = Piece::O;
        assert!(o.is_o());
    }


    #[test]
    fn x_printcheck()
    {
        let x = Piece::X;
        assert_eq!(print_piece(Some(x)), "X");
    }

    #[test]
    fn o_printcheck()
    {
        let o = Piece::O;
        assert_eq!(print_piece(Some(o)), "O");
    }


}