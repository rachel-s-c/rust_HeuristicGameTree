pub mod checkers;
pub mod connect4;
pub mod general_game;
pub mod tictactoe;

use core::isize::{MAX, MIN};


#[allow(unused)]

pub trait HeuristicGameTree: Clone {
    type Move: Clone + Sized;
    //type Heuristic: PartialOrd;
    // Should be a type Heuristic that can be any type that can be compared; doing isize for now

    /// All the possible moves that the computer agent and player can make based on the existing
    /// game board are determined with this function.
    /// Returns an iterator, where all the items are possible moves
    fn possible_moves(&self) -> Vec<Self::Move>;

    /// The computation of heuristics differ with the type of game
    /// This makes the individually unique heuristics into a value of type isize
    fn heuristic(&self) -> isize;

    /// After determining the best move to make, the computer agent will execute the move
    /// The game is updated with the computer agent's move and returns true if the next player is the opponent
    /// The player is now free to make their turn
    fn execute_move(&mut self, next_move: &Self::Move, is_opponent: bool) -> bool;


    /// Function: Minimax with alpha-beta pruning
    /// Gets the possible moves (i.e. children)
    /// Makes executes each move on a copy of the game
    /// Gets the heuristic of each game copy executed on a next move by calling minimax again
    ///
    /// # Arguments
    /// * `depth` - A usize that holds the depth of the minimax tree
    ///             (it is the number of moves that the computer agent will plan ahead, and it
    ///             represents the the difficulty of the game)
    ///
    /// * `is_opponent` - A boolean that represents whether it is the player or computer's turn
    ///
    /// Returns the best move (i.e. the move corresponding to the best heuristic)
    fn minimax_search(&mut self, depth: usize, is_opponent: bool) -> Option<Self::Move> {
        let mut best_move = (None, MIN); // We're going to maximize heuristic
        if depth > 0 {
            for mymove in self.possible_moves() {
                let mut next_state = self.clone();
                let opp = next_state.execute_move(&mymove.clone(), is_opponent); // Need to clone, standard procedure with minimax
                let h = next_state.minimax_helper(depth - 1, opp, MAX, MIN);
                if h > best_move.1 {
                    best_move = (Some(mymove), h);
                }
            }
        } else {
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
    fn minimax_helper(
        &mut self,
        depth: usize,
        is_opponent: bool,
        mut alpha: isize,
        mut beta: isize,
    ) -> isize {
        let mut current_heuristic = self.heuristic();
        if depth > 0 {
            // End of depth, return
            let mut child_heuristic = if is_opponent { MIN } else { MAX };
            for mymove in self.possible_moves() {
                let mut next_state = self.clone();
                let opp = next_state.execute_move(&mymove.clone(), is_opponent);
                let h = next_state.minimax_helper(depth - 1, opp, alpha, beta);
                if (h > child_heuristic && is_opponent) || (h < child_heuristic && !is_opponent) {
                    child_heuristic = h;
                }
                if is_opponent && h > alpha {
                    alpha = h;
                } else if !is_opponent && h > beta {
                    beta = h;
                }
                if beta < alpha {
                    break;
                }
            }
            if (child_heuristic > MIN && is_opponent) || (child_heuristic < MAX && !is_opponent) {
                current_heuristic = child_heuristic;
            }
        }
        current_heuristic
    }
}
