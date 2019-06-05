use crate::general_game::print_piece;
use crate::general_game::Piece;
use std::cmp::max;
use std::io::{stdin, stdout, Write};
use super::*;
use crate::minimax;

impl<'a> HeuristicGameTree for ConGame {
    type Move = usize;
    fn possible_moves(&self) -> Box<Iterator<Item = Self::Move> + '_> {
        let mut list = Vec::new();
        for i in 0..7 {
            if self.board[i][5].is_none() {
                list.push(i);
            }
        }
        Box::new(list.into_iter())
    }
    fn heuristic(&self) -> isize {
        let mut x_streak = 0;
        let mut o_streak = 0;
        // First check for wins
        let mut mutableself = self.clone();
        'outer: for a in 0..6 {
            for b in 0..5 {
                if mutableself.board[a][b] == Some(Piece::X) {
                    let cur = mutableself.check_win_and_length(a, b, Piece::X).1;
                    x_streak = max(cur, x_streak);
                    if x_streak >= 4
                    {
                        x_streak = 4;
                        break 'outer;
                    }
                }
            }
        }
            'outer2: for a in 0..6 {
                for b in 0..5 {
                    if mutableself.board[a][b] == Some(Piece::O) {
                        let cur = mutableself.check_win_and_length(a, b, Piece::O).1;
                        o_streak = max(cur, o_streak);
                        if o_streak >= 4
                        {
                            o_streak = 4;
                            break 'outer2;
                        }
                    }
                }
            }

        if x_streak == 4
        {
            x_streak
        }
        else if o_streak == 4
        {
            -o_streak
        }
        else {
            x_streak - o_streak
        }

    }
    fn execute_move(&mut self, next_move: &Self::Move, is_opponent: bool) -> bool{
        let (_val, loc) = self.clone().validmove(next_move + 1);
        self.store_move(
            *next_move,
            loc,
            if is_opponent { Piece::X } else { Piece::O },
        );
        !is_opponent
    }
}

//---------------------------ConnectGame----------------------------------------------

#[derive(Clone)]
struct ConGame {
    board: [[Option<Piece>; 6]; 7],
    winner: Option<Piece>,
}

impl ConGame {
    fn new() -> Self {
        ConGame {
            board: [[None; 6]; 7],
            winner: None,
        }
    }

    fn printboard(&mut self) {
        println!("1  2  3  4  5  6  7");
        for i in (0..6).rev() {
            println!(
                "{}  {}  {}  {}  {}  {}  {}",
                print_piece(self.board[0][i]),
                print_piece(self.board[1][i]),
                print_piece(self.board[2][i]),
                print_piece(self.board[3][i]),
                print_piece(self.board[4][i]),
                print_piece(self.board[5][i]),
                print_piece(self.board[6][i])
            );
        }
        println!("____________________");
    }

    fn validmove(self, col: usize) -> (bool, usize) {
        if col >= 1 && col <= 7 {
            let firstvec = col - 1;

            for i in 0..6 {
                if self.board[firstvec][i].is_none() {
                    return (true, i);
                }
            }
        }
        (false, 10)
    }

    fn store_move(&mut self, col: usize, row: usize, player: Piece) {
        self.board[col][row] = Some(player);
    }
    fn check_win_and_length(&mut self, col: usize, row: usize, player: Piece) -> (bool, isize) {
        let mut lengths = Vec::new();
        lengths.push(self.horizontal(col, row, player));
        lengths.push(self.vertical(col, row, player));
        lengths.push(self.lef_diag(col, row, player));
        lengths.push(self.right_diag(col, row, player));
        let longest = *lengths.iter().max().unwrap();
        let win = {
            if longest >= 4 {
                true
            } else {
                false
            }
        };
        (win, longest)
    }

    fn horizontal(&self, col: usize, row: usize, player: Piece) -> isize {
        let mut in_row = 1;

        if col != 0 {
            for i in (0..=col - 1).rev() {
                if self.board[i][row] == Some(player) {
                    in_row += 1;
                } else {
                    break;
                }
            }
        }
        for i in col + 1..7 {
            if i <= 6 {
                if self.board[i][row] == Some(player) {
                    in_row += 1;
                } else {
                    break;
                }
            }
        }
        in_row
    }

    fn vertical(&self, col: usize, row: usize, player: Piece) -> isize {
        let mut in_row = 1;

        if row != 0 {
            for i in (0..=row - 1).rev() {
                if self.board[col][i] == Some(player) {
                    in_row += 1;
                } else {
                    break;
                }
            }
        }
        for i in row + 1..6 {
            if i <= 5 {
                if self.board[col][i] == Some(player) {
                    in_row += 1;
                } else {
                    break;
                }
            }
        }
        in_row
    }

    fn lef_diag(&self, col: usize, row: usize, player: Piece) -> isize {
        let mut in_row = 1;
        for i in 1..5 {
            if row >= i && col >= i {
                let adj_row = row - i;
                let adj_col = col - i;
                if self.board[adj_col][adj_row] == Some(player) {
                    in_row += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        for i in 1..5 {
            let adj_row = row + i;
            let adj_col = col + i;

            if adj_col <= 6 && adj_row <= 5 {
                if self.board[adj_col][adj_row] == Some(player) {
                    in_row += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        in_row
    }

    fn right_diag(&self, col: usize, row: usize, player: Piece) -> isize {
        let mut in_row = 1;
        for i in 1..5 {
            if col >= i {
                let adj_row = row + i;
                let adj_col = col - i;

                if adj_row <= 5 {
                    if self.board[adj_col][adj_row] == Some(player) {
                        in_row += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        for i in 1..5 {
            if row >= i {
                let adj_row = row - i;
                let adj_col = col + i;

                if adj_col <= 6 {
                    if self.board[adj_col][adj_row] == Some(player) {
                        in_row += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        in_row
    }

    fn board_not_full(&self) -> bool {
        for pie in self.board.iter() {
            for piec in pie.iter() {
                if piec.is_none() {
                    return true;
                }
            }
        }
        false
    }
}

/// Starts the Connect4 game
/// # Arguments
///
/// * `difficulty` - A usize that holds the difficulty of the game. This value is passed as
///                  an argument to minimax search, determining the depth of the minimax search
///                  tree (i.e. the number of steps ahead that the AI agent should look ahead when
///                  determining its move)
pub fn start_con(difficulty: usize) {
    let mut new_game = ConGame::new();

    while new_game.winner == None && new_game.board_not_full() {
        println!("Where do you want to put your X? (Only input col)");
        new_game.printboard();
        let mut loc = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut loc)
            .expect("Did not enter a correct string");
        let mut loc = loc.split_whitespace();
        if loc.clone().count() == 1 {
            let col = loc.next().unwrap().parse().unwrap();
            let (valid, row) = new_game.clone().validmove(col); //throw error
            if valid {
                // pos is our move, store_move is our execute
                new_game.store_move(col - 1, row, Piece::X);
                if new_game.check_win_and_length(col - 1, row, Piece::X).0 {
                    new_game.winner = Some(Piece::X);
                } else {
                    let next_move = minimax::minimax_search(&new_game, 3 * difficulty, true);
                    if let Some(m) = next_move {
                        let (_val, loc) = new_game.clone().validmove(m + 1);
                        new_game.store_move(m, loc, Piece::O);
                        if new_game.check_win_and_length(m, loc, Piece::O).0 {
                            new_game.winner = Some(Piece::O);
                        }
                    }
                }
            } else {
                println!("That is not a valid move! Try again");
            }
        } else {
            println!("You did not input your move correctly! Try again");
        }
    }
    new_game.printboard();
    if new_game.winner.is_some() {
        println!("{} WON THE GAME!", print_piece(new_game.winner));
    } else {
        println!("TIE!");
    }
}

//---------------------------ConnectGame----------------------------------------------

#[cfg(test)]
mod con_tests {
    use super::print_piece;
    use super::ConGame;
    use super::Piece;

    #[test]
    fn new_con_test() {
        let con_1 = ConGame::new();
        assert_eq!(con_1.board[1][1], None);
    }

    #[test]
    fn store_con_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, Piece::X);
        assert_eq!(con_1.board[5][1].unwrap(), Piece::X);
    }

    #[test]
    fn print_piece_x_con_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, Piece::X);
        assert_eq!(print_piece(con_1.board[5][1]), "X");
    }

    #[test]
    fn valid_con_test() {
        let con_1 = ConGame::new();
        let (a, _b) = con_1.validmove(5);
        assert!(a);
    }

    #[test]
    fn validrow_con_test() {
        let con_1 = ConGame::new();
        let (_a, b) = con_1.validmove(5);
        assert_eq!(b, 0);
    }

    #[test]
    fn invalid_con_test() {
        let con_1 = ConGame::new();
        let (a, _b) = con_1.validmove(8);
        assert!(!a);
    }

    #[test]
    fn board_not_full_test() {
        let mut con_1 = ConGame::new();
        for a in 0..7 {
            for b in 0..6 {
                con_1.store_move(a, b, Piece::X);
            }
        }
        assert_eq!(con_1.board_not_full(), false);
    }

    #[test]
    fn board_not_full2_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, Piece::X);
        assert_eq!(con_1.board_not_full(), true);
    }

    #[test]
    fn lose_con_test() {
        let mut con_1 = ConGame::new();
        let a = con_1.check_win_and_length(5, 1, Piece::X).0;
        assert!(!a);
    }

    #[test]
    fn win_vert_con_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(1, 1, Piece::X);
        con_1.store_move(1, 2, Piece::X);
        con_1.store_move(1, 3, Piece::X);
        con_1.store_move(1, 4, Piece::X);
        let a = con_1.check_win_and_length(1, 4, Piece::X).0;
        assert!(a);
    }

    #[test]
    fn win_horiz_con_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(1, 1, Piece::X);
        con_1.store_move(2, 1, Piece::X);
        con_1.store_move(3, 1, Piece::X);
        con_1.store_move(4, 1, Piece::X);
        let a = con_1.check_win_and_length(4, 1, Piece::X).0;
        assert!(a);
    }

    #[test]
    fn horiz_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(1, 1, Piece::X);
        con_1.store_move(2, 1, Piece::X);
        con_1.store_move(3, 1, Piece::X);
        con_1.store_move(4, 1, Piece::X);
        let a = con_1.horizontal(3, 1, Piece::X);
        assert_eq!(a, 4);
    }

    #[test]
    fn vert_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(1, 1, Piece::X);
        con_1.store_move(1, 2, Piece::X);
        con_1.store_move(1, 3, Piece::X);
        con_1.store_move(1, 4, Piece::X);
        let a = con_1.vertical(1, 2, Piece::X);
        assert_eq!(a, 4);
    }

    #[test]
    fn ldiag_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(0, 0, Piece::X);
        con_1.store_move(1, 1, Piece::X);
        con_1.store_move(2, 2, Piece::X);
        con_1.store_move(3, 3, Piece::X);
        let a = con_1.lef_diag(1, 1, Piece::X);
        assert_eq!(a, 4);
    }

    #[test]
    fn rdiag_test() {
        let mut con_1 = ConGame::new();
        con_1.store_move(0, 3, Piece::X);
        con_1.store_move(1, 2, Piece::X);
        con_1.store_move(2, 1, Piece::X);
        con_1.store_move(3, 0, Piece::X);
        let a = con_1.right_diag(2, 1, Piece::X);
        assert_eq!(a, 4);
    }
}
