pub mod checkers;
pub mod connect4;
pub mod general_game;
pub mod minimax;
pub mod tictactoe;

#[allow(unused)]
/// A trait for the ability to make a game tree of possible moves with values of heuristic evaluations.
///
/// A heuristic is an approximation that is typically fast and used to aid in optimization problems.
/// In this context, heuristics are used to “rate” board positions based on local information.
/// It provides an informed way to guess which neighbor of a node will lead to a goal.
/// Using heuristics is a common way to measure progress in a game. When dealing with game trees,
/// the heuristic function is generally referred to as the evaluation function, or the static
/// evaluator. The static evaluation takes in a board position, and gives it a score.
/// The higher the score, the better it is for you, the lower, the better for the opponent.
///
/// # How can I implement HeuristicGameTree?
/// HeuristicGameTree requires the possible_moves, heuristic, execute_move methods to be implemented.
/// An simple example implementation for a connect4 game is:
/// ```
/// use heuristic_game_tree::HeuristicGameTree;
///
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
///}
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
/// ```

pub trait HeuristicGameTree: Clone {
    type Move: Clone + Sized;
    //type Heuristic: PartialOrd;

    /// All the possible moves that the computer agent and player can make based on the existing
    /// game board are determined with this function.
    /// Returns a box of an iterator, where all the items are possible moves
    fn possible_moves(&self) -> Box<Iterator<Item = Self::Move> + '_>;

    /// The computation of heuristics differ with the type of game
    /// This makes the individually unique heuristics into a value of type isize
    // Should be a type Heuristic that can be any type that can be compared; doing isize for now
    fn heuristic(&self) -> isize;

    /// After determining the best move to make, the computer agent will execute the move
    /// The game is updated with the computer agent's move and returns true if the next player is the opponent
    /// The player is now free to make their turn
    fn execute_move(&mut self, next_move: &Self::Move, is_opponent: bool) -> bool;
}
