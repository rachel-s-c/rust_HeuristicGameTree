
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
///
/// # Examples
/// Basic usage:
/// ```
/// #[derive(PartialEq, Copy, Clone, Debug)]
/// pub enum Piece {
///    /// An 'X' piece
///    X,
///    /// An 'O' piece
///    O,
///}
/// #[derive(Clone)]
/// struct Connect4Game {
///    board: [[Option<Piece>; 6]; 7],
///    winner: Option<Piece>,
/// }
/// impl Connect4Game {
///    fn new() -> Self {
///        Connect4Game {
///            board: [[None; 6]; 7],
///            winner: None,
///        }
///    }
/// }
/// # use heuristic_game_tree::HeuristicGameTree;
/// impl HeuristicGameTree for Connect4Game{
///     type Move = usize;
///
///     // Returns a box that contains a pointer to an iterator of all the moves that can be made
///     fn possible_moves(&self) -> Box<Iterator<Item = Self::Move> + '_> {
///        let mut list = Vec::new();
///        for i in 0..7 { list.push(i);}
///        Box::new(list.into_iter())
/// }
///
///     // Returns the value of the heuristic that determines if the move is advantageous.
///     fn heuristic(&self) -> isize {4}
///
///     // Returns whether the move has been made.
///     fn execute_move(&mut self, next_move: &Self::Move, is_opponent: bool) -> bool{true}
/// }
///
/// # use heuristic_game_tree::minimax::minimax_search;
/// let mut new_game = Connect4Game::new();
/// let difficulty: usize = 2;
/// let boolean = true;
///
/// // next_move is an Option<usize>
/// // since we are beginning with an empty Connect4Game board, the best move is the first move (i.e. 0)
/// let next_move = minimax_search(&new_game, 3 * difficulty, boolean);
/// assert_eq!(0, next_move.unwrap());
/// ```
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


// Need a helper because the client shouldn't provide alpha and beta
// also nice because we don't have to have return valus of structs/tuples, can just do an isize
// that corresponds to the best value for the immediately next move
fn minimax_helper<G>(game: &G, depth: usize, is_opponent: bool, mut alpha: isize, mut beta: isize) -> isize
    where
        G: HeuristicGameTree{
    let mut current_heuristic = game.heuristic();
    if depth > 0 {
        // End of depth, return
        let mut child_heuristic = if is_opponent { MIN } else { MAX };
        for mymove in game.possible_moves() {
            let mut next_state = game.clone();
            let opp = next_state.execute_move(&mymove.clone(), is_opponent);
            let h = minimax_helper(&next_state,depth - 1, opp, alpha, beta);
            if (h > child_heuristic && is_opponent) || (h < child_heuristic && !is_opponent) {
                child_heuristic = h;
            }
            if is_opponent && h <= alpha {
                alpha = h;
            } else if !is_opponent && h < beta {
                beta = h;
            }
            if beta <= alpha {
                break;
            }
        }
        if (child_heuristic > MIN && is_opponent) || (child_heuristic < MAX && !is_opponent) {
            current_heuristic = child_heuristic;
        }
    }
    current_heuristic
}