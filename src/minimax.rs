
use super::HeuristicGameTree;
use core::isize::{MAX, MIN};


/// Function: Minimax with alpha-beta pruning.
/// Minimax is a decision rule that minimizes the possible loss for a worst case (maximum loss) scenario,
/// and maximizes the possible gain for a best case (maximum win) scenario. This function is
/// deterministic and does not take into account uncertainty (random chance elements).
///
/// This function gets all the possible moves (i.e. children), then executes each move on a
/// copy of the game. It gets the heuristic of each game copy executed on a next move by calling
/// minimax recursively.
///
/// The game tree is cut off at a certain maximum depth, d (called a d-ply search), depending on the
/// difficulty rating picked by the player. The heuristic function to those positions are at the
/// bottom nodes of the tree, so instead of just Win, Loss, Tie, there is a heuristic score.
///
/// # Arguments
/// * `depth` - A usize that holds the depth of the minimax tree
///             (it is the number of moves that the computer agent will plan ahead, and it
///             represents the the difficulty of the game)
///
/// * `is_opponent` - A boolean that represents whether it is the player or computer's turn
///
/// Returns the best move (i.e. the move corresponding to the best heuristic)
pub fn minimax_search<G>(game: &G, depth: usize, is_opponent: bool) -> Option<G::Move>
    where
        G: HeuristicGameTree{
        let mut best_move = (None, MIN); // We're going to maximize heuristic
        if depth > 0 {
            for mymove in game.possible_moves() {
                let mut next_state = game.clone();
                let opp = next_state.execute_move(&mymove, is_opponent); // Need to clone, standard procedure with minimax
                let h = minimax_helper(&next_state,depth - 1, opp, MAX, MIN);
                if h > best_move.1 {
                    best_move = (Some(mymove), h);
                }
            }
        } else {
            // Choose first available move.
            let mut moves = game.possible_moves();
            // if moves.len() == 0 {
            //     return None;
            // }
            // return Some(moves[0].clone());
            return moves.next();
        }
        //println!("{}",best_move.1);
        best_move.0 // Return the move that corresponds with best heuristic
    }


// Need a helper cause the client shouldn't provide alpha and beta
// also nice because we don't have to have return valus of structs/tuples, can just do an isize
// that corresponds to the best value for the immediately next move
fn minimax_helper<G>(game: &G, depth: usize, is_opponent: bool, mut alpha: isize, mut beta: isize) -> isize
    where
        G: HeuristicGameTree{
    let mut current_heuristic = game.heuristic();
    if depth > 0 {
        // End of depth, return
        // let mut child_heuristic = if is_opponent { MIN } else { MAX };
        for mymove in game.possible_moves() {
            let mut next_state = game.clone();
            let opp = next_state.execute_move(&mymove.clone(), is_opponent);
            let h = minimax_helper(&next_state,depth - 1, opp, alpha, beta);
            if (h > current_heuristic && is_opponent) || (h < current_heuristic && !is_opponent) {
                current_heuristic = h;
            }
            if is_opponent && h > alpha {
                alpha = h;
            } else if !is_opponent && h < beta {
                beta = h;
            }
            if beta < alpha {
                break;
            }
        }
        // if (child_heuristic > MIN && is_opponent) || (child_heuristic < MAX && !is_opponent) {
        //     current_heuristic = child_heuristic;
        // }
    }
    current_heuristic
}




/*
use std::f64::INFINITY;


#[allow(unused)]
pub trait HeuristicGameTree {
    type Game;
    type Move;
    type Heuristic;



}




/// Each unique position on the game board
///
/// # Fields
/// * `value` - An f64 that represents the heuristic (i.e. how good that position is)
///
/// * `children` - A vector of positions resulting from possible moves that can be made
///                once this move is executed
pub struct Position {
    value : f64,
    children: Vec<Position>,
}

impl Position {

    /// Creates a new position with value of 0.0 and an empty vector for children
    pub fn new() -> Position {
        Position {
            value: 0.0,
            children: Vec::new(),
        }
    }

    /// Creates an iterator of all subsequent possible moves (i.e. 'children' moves)
    pub fn children(&mut self) -> impl Iter<Item = Position> {
        self.children.iter()
    }

    /// Creates a heuristic value for the position
    pub fn heuristic(&self) -> f64 {
        self.value
    }

    /// Returns a new board position after executing move
    pub fn execute_move(&mut self) -> Self {}
}











// supposing we pass this function a hashmap, where the key is the move, and the value is the
// vector of all the possible next steps that can be reached from this move
// returns the position and its score
// alpha would initially be set to -infinity, and beta would be +infinity
pub fn minimax(depth: usize, is_maximizing_player: bool, pos: Position,
               h: usize, mut alpha: f64, mut beta: f64) -> Position
{
    // Terminating condition. i.e
    // leaf node is reached
    if depth == h {
        return pos; //  should return heuristic, no?
    }

    // If current move is maximizer,
    // find the maximum attainable
    // value
    if is_maximizing_player{
        let mut max_eval = Position::new();
        max_eval.value = -INFINITY;

        for child in pos.children() { // And if there are no children? returns -infinity?
            let eval = minimax(depth+1, false, child, h, alpha, beta);
            let eval_value = eval.heuristic();
            if max_eval.value < eval.value {
                max_eval = eval;
            }
            if eval_value > alpha {
                alpha = eval_value;
            }
            if beta <= alpha {break}
        }
        return max_eval
    }

    // Else (If current move is Minimizer), find the minimum
    // attainable value
    else {
        let mut min_eval = Position::new();
        min_eval.value = INFINITY;
        for child in pos.children {
            let eval = minimax(depth+1, false, child, h, alpha, beta);
            let eval_value = eval.value;
            if min_eval.value < eval.value {
                min_eval = eval;
            }
            if eval_value < beta {
                beta = eval_value;
            }
            if beta <= alpha {break}
        }
        return min_eval
    }
}

*/