use std::io::{stdin,stdout,Write};
use crate::general_game::{Piece};

//---------------------------ConnectGame----------------------------------------------

#[derive (Clone)]
struct ConGame<'a>
{
    board: Vec<Vec<&'a Piece>>,
    winner: Option<Piece>,
    moves: usize,
}

impl<'a> ConGame<'a>
{
    fn new() -> Self
    {
        ConGame {
            board: vec![vec![&Piece::None; 6]; 7],
            winner: None,
            moves: 0,
        }
    }

    fn printboard(&mut self)
    {
        println!("1  2  3  4  5  6  7");
        println!("{}  {}  {}  {}  {}  {}  {}", self.board[0][5].val(), self.board[1][5].val(),
                 self.board[2][5].val(), self.board[3][5].val(), self.board[4][5].val(),
                 self.board[5][5].val(), self.board[6][5].val());
        println!("{}  {}  {}  {}  {}  {}  {}", self.board[0][4].val(), self.board[1][4].val(),
                 self.board[2][4].val(), self.board[3][4].val(), self.board[4][4].val(),
                 self.board[5][4].val(), self.board[6][4].val());
        println!("{}  {}  {}  {}  {}  {}  {}", self.board[0][3].val(), self.board[1][3].val(),
                 self.board[2][3].val(), self.board[3][3].val(), self.board[4][3].val(),
                 self.board[5][3].val(), self.board[6][3].val());
        println!("{}  {}  {}  {}  {}  {}  {}", self.board[0][2].val(), self.board[1][2].val(),
                 self.board[2][2].val(), self.board[3][2].val(), self.board[4][2].val(),
                 self.board[5][2].val(), self.board[6][2].val());
        println!("{}  {}  {}  {}  {}  {}  {}", self.board[0][1].val(), self.board[1][1].val(),
                 self.board[2][1].val(), self.board[3][1].val(), self.board[4][1].val(),
                 self.board[5][1].val(), self.board[6][1].val());
        println!("{}  {}  {}  {}  {}  {}  {}", self.board[0][0].val(), self.board[1][0].val(),
                 self.board[2][0].val(), self.board[3][0].val(), self.board[4][0].val(),
                 self.board[5][0].val(), self.board[6][0].val());
        println!("____________________");
    }

    fn validmove(self, col: usize) -> (bool, usize)
    {
        if col >= 1 && col <= 7 {
            let firstvec = col - 1;

            for i in 0..5
                {
                    if self.board[firstvec][i] == &Piece::None
                    {
                        return (true, i)
                    }
                };
        }
        (false, 10)
    }

    fn store_move(&mut self, col: usize, row: usize, player: &'a Piece) {
        println!("{} {}", col, row);
        self.board[col][row] = player;
        self.moves += 1;
        if self.moves == 42
        {
            self.winner = Some(Piece::Tie);
        }
    }
    fn check_win(&mut self, col: usize, row: usize, player: Piece) -> bool
    {
        let mut in_row = 1;
        if col != 0 {
            for i in (0..=col - 1).rev() //horizontal
                {
                    if self.board[i][row] == &player
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
        }
        for i in col+1..7
            {
                if i <= 6 {
                    if self.board[i][row] == &player
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
            }
        if in_row >= 4
        {
            return true;
        }
        in_row = 1;
        if row != 0 {
            for i in (0..=row - 1).rev() //vertical
                {
                    if self.board[col][i] == &player
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
        }
        for i in row+1..6
            {
                if i <= 5 {
                    if self.board[col][i] == &player
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
            }
        if in_row >= 4
        {
            return true;
        }
        in_row = 1;
        for i in 1..5 //l diag
            {
                if row >= i && col >= i
                {
                    let adj_row = row - i;
                    let adj_col = col - i;
                    if self.board[adj_col][adj_row] == &player
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
        for i in 1..5 //l diag
            {
                let adj_row = row + i;
                let adj_col = col + i;

                if adj_col <= 6 && adj_row <= 5
                {
                    if self.board[adj_col][adj_row] == &player
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
        if in_row >= 4
        {
            return true;
        }
        in_row = 1;
        for i in 1..5 //r diag
            {
                if col >= i
                {
                    let adj_row = row + i;
                    let adj_col = col - i;

                    if adj_row <= 5
                    {
                        if self.board[adj_col][adj_row] == &player
                        {
                            in_row += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
        for i in 1..5 //r diag
            {
                if row >= i
                {
                    let adj_row = row - i;
                    let adj_col = col + i;

                    if adj_col <= 6
                    {
                        if self.board[adj_col][adj_row] == &player
                        {
                            in_row += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
        if in_row >= 4
        {
            return true;
        }

        false
    }
}

pub fn start_con(difficulty: usize)
{
    let mut new_game = ConGame::new();

    while new_game.winner == None {
        println!("Where do you want to put your X? (Only input col)");
        new_game.printboard();
        let mut loc = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut loc).expect("Did not enter a correct string");
        let mut loc = loc.split_whitespace();
        if loc.clone().count() == 1 {
            let col =  loc.next().unwrap().parse().unwrap();
            let (valid, row) = new_game.clone().validmove(col); //throw error
            if valid
            {
                // pos is our move, store_move is our execute
                new_game.store_move(col-1, row, &Piece::X);
                if new_game.check_win(col-1, row, Piece::X)
                {
                    new_game.winner = Some(Piece::X);
                }
                else {
                    /* let next_move = new_game.minimax_search(difficulty * 3, true);
                     if let Some(m) = next_move {
                         new_game.store_move(m, &Piece::O);
                         if new_game.check_win(Piece::O) {
                             new_game.winner = Some(Piece::O);
                         }
                     } */
                    new_game.store_move(0, 0, &Piece::O);
                    if new_game.check_win(0, 0, Piece::O) {
                        new_game.winner = Some(Piece::O);
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
    println!("{} WON THE GAME!", new_game.winner.unwrap().val());
}

//---------------------------ConnectGame----------------------------------------------

#[cfg(test)]
mod con_tests {
    use super::Piece;
    use super::ConGame;

    #[test]
    fn new_con_test()
    {
        let con_1 = ConGame::new();
        assert_eq!(con_1.moves, 0);
    }

    #[test]
    fn store_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, &Piece::X);
        assert_eq!(con_1.board[5][1], &Piece::X);
    }

    #[test]
    fn val_x_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, &Piece::X);
        assert_eq!(con_1.board[5][1].val(), "X");
    }

    #[test]
    fn valid_con_test()
    {
        let mut con_1 = ConGame::new();
        let (a, b) = con_1.validmove(5);
        assert!(a);
    }

    #[test]
    fn validrow_con_test()
    {
        let mut con_1 = ConGame::new();
        let (a, b) = con_1.validmove(5);
        assert_eq!(b, 0);
    }

    #[test]
    fn invalid_con_test()
    {
        let mut con_1 = ConGame::new();
        let (a, b) = con_1.validmove(8);
        assert!(!a);
    }

    #[test]
    fn tie_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.moves += 41;
        con_1.store_move(5, 1, &Piece::X);
        assert_eq!(con_1.winner.unwrap(), Piece::Tie);
    }

    #[test]
    fn lose_con_test()
    {
        let mut con_1 = ConGame::new();
        let a = con_1.check_win(5, 1, Piece::X);
        assert!(!a);
    }

    #[test]
    fn win_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(1, 1,  &Piece::X);
        con_1.store_move(1, 2,  &Piece::X);
        con_1.store_move(1, 3, &Piece::X);
        con_1.store_move(1, 4, &Piece::X);
        let a = con_1.check_win(1, 4, Piece::X);
        assert!(a);
    }
}
