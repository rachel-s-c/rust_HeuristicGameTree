
use std::f64::INFINITY;

#[allow(unused)]
pub trait HeuristicGameTree {
    type Game;
    type Move;
    type Heuristic;

    fn possible_moves(&self) -> impl Iter<Item = Move>;
    fn heuristic(&self) -> Heuristic;
    fn execute_move(&mut self);

    fn minimax_search(&self, depth: usize, is_opponent: bool,) -> Move {
        // Gets the possible moves (i.e. children)
        // Makes executes each move on a copy of the game
        // Gets the heuristic of each game copy executed on a next move
        // by calling minimax again
        // Returns the move corresponding with the best heuristic


    }
}

// every position has a value and a vector of possible moves from that
pub struct Position {
    value : f64,
    children: Vec<Position>,
}

impl Position{
    pub fn new() -> Position {
        Position {
            value: 0.0,
            children: Vec::new(),
        }
    }
    pub fn children(&mut self) -> impl Iter<Item = Position> {
        self.children.iter()
    }
    pub fn heuristic(&self) -> f64 {
        self.value
    }
    pub fn execute_move(&mut self) -> Self {

    }
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

// this commented out code is the simple version with 2 branches
/*
// Returns the optimal value a maximizer can obtain.
// depth is current depth in game tree.
// nodeIndex is index of current node in scores[].
// isMax is true if current move is
// of maximizer, else false
// scores[] stores leaves of Game tree.
// h is maximum height of Game tree
pub fn minimax(depth: usize, nodeIndex: usize, isMaximizingPlayer: bool, scores: &Vec<usize>, h: usize) -> usize
{
    // Terminating condition. i.e
    // leaf node is reached
    if depth == h {
        return scores[nodeIndex];
    }

    // If current move is maximizer,
    // find the maximum attainable
    // value
    if isMaximizingPlayer {
        return max(minimax(depth+1, nodeIndex*2, false, scores, h),
                   minimax(depth+1, nodeIndex*2 + 1, false, scores, h));
    }

// Else (If current move is Minimizer), find the minimum
// attainable value
    else {
        return min(minimax(depth+1, nodeIndex*2, true, scores, h),
                   minimax(depth+1, nodeIndex*2 + 1, true, scores, h));
    }
}

pub fn test()
{
// The number of elements in scores must be
// a power of 2.
    println!("Hello, world!");
    let mut scores = vec![3, 5, 2, 9, 12, 5, 23, 23];
    //let n = sizeof(scores)/sizeof(scores[0])_f64;
    let n: f64 = 8.0;
    let h = n.log2() as usize;
    let res = minimax::minimax(0, 0, true, &scores, h);
    println!("The optimal value is : {}", res);
}

*/