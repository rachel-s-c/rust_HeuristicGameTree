use std::io::{stdin,stdout,Write};
use std::cmp::max;
use crate::general_game::{HeuristicGameTree, Piece};
use crate::general_game::print_piece;

const TICWINS: [[usize; 3]; 8] = [[0, 1, 2], [0, 3, 6], [0, 4, 8], [1, 4, 7], [2, 5, 8],
    [2, 4, 6], [3, 4, 5], [6, 7, 8]];


impl<'a> HeuristicGameTree for TicGame {
    type Move = usize;
    //type Heuristic = isize;
    // fn possible_moves(&self) -> Iterator<Item = Self::Move> {
    fn possible_moves(&self) -> Vec<Self::Move> {
        let mut list = Vec::new();
        for i in 0..9 {
            if self.board[i].is_none() {
                list.push(i);
            }
        }
        // list.iter()
        list
    }
    // MAKE THIS BETTER
    fn heuristic(&self) -> isize {
        // invariant: x_streak != o_streak != 3
        // keep track of best streak by each player
        //
        let mut x_streak = if self.move_count() > 0 {1} else {0};
        let mut o_streak = if self.move_count() > 1 {1} else {0};
        // First check for wins
        if self.board[4].is_some() {
            let center = self.board[4].clone();
            if center == self.board[0] && center == self.board[8] {
                if center == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[1] && center == self.board[7] {
                if center == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[2] && center == self.board[6] {
                if center == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[3] && center == self.board[5] {
                if center == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        if self.board[0].is_some() {
            let corner = self.board[0].clone();
            if corner == self.board[1] && corner == self.board[2] {
                if corner == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if corner == self.board[3] && corner == self.board[6] {
                if corner == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        if self.board[8].is_some() {
            let corner = self.board[8].clone();
            if corner == self.board[5] && corner == self.board[2] {
                if corner == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if corner == self.board[7] && corner == self.board[6] {
                if corner == Some(Piece::X) {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        // Do pairs
        if self.board[4].is_some() {
            let center = self.board[4].clone();
            for i in 0..9 {
                if i != 4 && center == self.board[i] { // found a pair
                    if center == Some(Piece::X) {
                        x_streak = max(x_streak,2);
                    }
                    else {
                        o_streak = max(o_streak,2);
                    }
                }
            }
        }
        let corner = self.board[0].clone();
        if self.board[0].is_some() {
            if corner == self.board[1] || corner == self.board[3] {
                if corner == Some(Piece::X) {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[2].is_some() {
           // let corner = self.board[0].clone();
            if corner == self.board[1] || corner == self.board[5] {
                if corner == Some(Piece::X) {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[6].is_some() {
          //  let corner = self.board[0].clone();
            if corner == self.board[7] || corner == self.board[3] {
                if corner == Some(Piece::X) {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[8].is_some() {
          //  let corner = self.board[0].clone();
            if corner == self.board[7] || corner == self.board[5] {
                if corner == Some(Piece::X) {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        x_streak - o_streak // Why is this backwards?
    }
    fn execute_move(&mut self, next_move: &Self::Move, is_opponent: bool) {
        self.store_move(*next_move, if is_opponent {Piece::O} else {Piece::X});
    }
}
//------------------------------------TicGame-----------------------------------------

#[derive (Clone)]
struct TicGame
{
    board: [Option<Piece>; 9],
    winner: Option<Piece>,
}

impl<'a> TicGame
{
    pub fn new() -> Self
    {

        TicGame {
            board: [None; 9],
            winner: None,
        }
    }

    fn printboard(& mut self)
    {
        println!("  1  2  3");
        println!("A {}  {}  {}", print_piece(self.board[0]), print_piece(self.board[1]), print_piece(self.board[2]));
        println!("B {}  {}  {}", print_piece(self.board[3]), print_piece(self.board[4]), print_piece(self.board[5]));
        println!("C {}  {}  {}", print_piece(self.board[6]), print_piece(self.board[7]), print_piece(self.board[8]));
    }

    fn validmove(self, row: &'a str, col: usize) -> (bool, usize)
    {
        if (row == "A" || row == "B" || row == "C") && (col == 1 || col == 2 || col == 3) {
            let int = match row {
                "A" => 0,
                "B" => 3,
                "C" => 6,
                _ => 100,
            };

            let num = int + col - 1;

            if self.board[num].is_none() {
                return (true, num)
            }
        }

        (false, 10)
    }

    fn store_move(&mut self, position:usize, player: Piece){
        self.board[position] = Some(player);
    }

    fn check_win(&mut self, player: Piece) -> bool
    {
        for vecs in TICWINS.iter()
            {
                let mut in_row = 0;
                for index in vecs.iter()
                    {
                        if self.board[*index] == Some(player)
                        {
                            in_row += 1;
                        }
                    }
                if in_row == 3
                {
                    return true
                }
            }
        false
    }

    fn move_count(&self) -> usize
    {
        let mut count = 0;
        for piec in self.board.iter()
            {
                if piec.is_some()
                {
                    count += 1;
                }
            }
        count
    }

    fn board_not_full(&self) -> bool
    {
        for piec in self.board.iter()
            {
                if piec.is_none()
                {
                    return true
                }
            }
        false
    }

}

pub fn start_tic(difficulty: usize)
{
    let mut new_game = TicGame::new();

    while new_game.winner == None && new_game.board_not_full() {
        println!("Where do you want to put your X? Input format: row(space)column e.g. A 1");
        new_game.printboard();
        let mut loc = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut loc).expect("Did not enter a correct string");
        let mut loc = loc.split_whitespace();
        if loc.clone().count() == 2 {
            let row =  loc.next().unwrap();
            let col =  loc.next().unwrap();
            let (valid, pos) = new_game.clone().validmove(row, col.parse().unwrap()); //throw error
            if valid
            {
                // pos is our move, store_move is our execute
                new_game.store_move(pos, Piece::X);
                if new_game.check_win(Piece::X)
                {
                    new_game.winner = Some(Piece::X);
                } else {
                    let next_move = new_game.minimax_search(difficulty * 3, true);
                    if let Some(m) = next_move {
                        new_game.store_move(m, Piece::O);
                        if new_game.check_win(Piece::O) {
                            new_game.winner = Some(Piece::O);
                        }
                    }
                }
            }
            else {
                println!("That is not a valid move! Try again");
            }
        }
        else {
            println!("You did not input your move correctly! Try again");
        }
    }
    new_game.printboard();
    if new_game.winner.is_some() {
        println!("{} WON THE GAME!", print_piece(new_game.winner));
    }
    else { println!("TIE!"); }
}

//------------------------------------TicGame-----------------------------------------

#[cfg(test)]
mod tic_tests {
    use super::TicGame;
    use super::Piece;
    use super::print_piece;

    #[test]
    fn new_tic_test()
    {
        let tic_1 = TicGame::new();
        assert_eq!(tic_1.board[1], None);
    }

    #[test]
    fn store_tic_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(5, Piece::X);
        assert_eq!(tic_1.board[5].unwrap(), Piece::X);
    }

    #[test]
    fn print_piece_x_tic_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(5, Piece::X);
        assert_eq!(print_piece(tic_1.board[5]), "X");
    }

    #[test]
    fn print_piece_o_tic_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(5, Piece::O);
        assert_eq!(print_piece(tic_1.board[5]), "O");
    }

    #[test]
    fn print_piece_empty_tic_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(5, Piece::X);
        assert_eq!(print_piece(tic_1.board[6]), " ");
    }

    #[test]
    fn move_count_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(5, Piece::X);
        assert_eq!(tic_1.move_count(), 1);
    }

    #[test]
    fn board_not_full_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(5, Piece::X);
        assert_eq!(tic_1.board_not_full(), true);
    }

    #[test]
    fn board_not_full2_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(0, Piece::X);
        tic_1.store_move(1, Piece::O);
        tic_1.store_move(2, Piece::X);
        tic_1.store_move(3, Piece::O);
        tic_1.store_move(4, Piece::X);
        tic_1.store_move(5, Piece::O);
        tic_1.store_move(6, Piece::O);
        tic_1.store_move(7, Piece::X);
        tic_1.store_move(8, Piece::O);
        assert_eq!(tic_1.board_not_full(), false);
    }

    #[test]
    fn valid_tic_test()
    {
        let mut tic_1 = TicGame::new();
        let (a, b) = tic_1.validmove("A", 2);
        assert!(a);
    }

    #[test]
    fn invalid_tic_test()
    {
        let mut tic_1 = TicGame::new();
        let (a, b) = tic_1.validmove("D", 2);
        assert!(!a);
    }

    #[test]
    fn lose_tic_test()
    {
        let mut tic_1 = TicGame::new();
        let a = tic_1.check_win(Piece::X);
        assert!(!a);
    }

    #[test]
    fn win_tic_test()
    {
        let mut tic_1 = TicGame::new();
        tic_1.store_move(0, Piece::X);
        tic_1.store_move(1, Piece::X);
        tic_1.store_move(2, Piece::X);
        let a = tic_1.check_win(Piece::X);
        assert!(a);
    }
}