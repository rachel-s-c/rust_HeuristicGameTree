use std::io::{stdin,stdout,Write};
use std::cmp::max;
use crate::general_game::{HeuristicGameTree, Piece};
use crate::general_game::print_piece;

impl<'a> HeuristicGameTree for ConGame {
    type Move = usize;
    fn possible_moves(&mut self) -> Vec<Self::Move> {
        let mut list = Vec::new();
        for i in 0..7 {
            if self.board[i][5].is_none() {
                list.push(i);
            }
        }
        list
    }
    // MAKE THIS BETTER
    fn heuristic(&self) -> isize {

        let mut x_streak = 1;
        let mut o_streak = 1;
        // First check for wins
        let mut mutableself = self.clone();
        'outer: for a in 0..6
            {
                for b in 0..5
                    {
                        if mutableself.board[a][b] == Some(Piece::X)
                        {
                            let cur = mutableself.longest_row(a, b, Piece::X);
                            x_streak = max(cur, x_streak);
                            if x_streak >= 4
                            {
                                break 'outer;
                            }
                        }
                    }
            }

        'outer2: for a in 0..6
            {
                for b in 0..5
                    {
                        if mutableself.board[a][b] == Some(Piece::O)
                        {
                            let cur = mutableself.longest_row(a, b, Piece::O);
                            o_streak = max(cur, o_streak);
                            if o_streak >= 4
                            {
                                break 'outer2;
                            }
                        }
                    }
            }


        x_streak - o_streak // Why is this backwards?
    }
    fn execute_move(&mut self, next_move: Self::Move, is_opponent: bool) {
        let (val, loc) = self.clone().validmove(next_move + 1);
        self.store_move(next_move, loc, if is_opponent {Piece::O} else {Piece::X});
    }
}





//---------------------------ConnectGame----------------------------------------------

#[derive (Clone)]
struct ConGame
{
    board: [[Option<Piece>; 6]; 7],
    winner: Option<Piece>,
}

impl ConGame
{
    fn new() -> Self
    {
        ConGame {
            board: [[None; 6]; 7],
            winner: None,
        }
    }

    fn printboard(&mut self)
    {
        println!("1  2  3  4  5  6  7");
        println!("{}  {}  {}  {}  {}  {}  {}", print_piece(self.board[0][5]), print_piece(self.board[1][5]),
                 print_piece(self.board[2][5]), print_piece(self.board[3][5]), print_piece(self.board[4][5]),
                 print_piece(self.board[5][5]), print_piece(self.board[6][5]));
        println!("{}  {}  {}  {}  {}  {}  {}", print_piece(self.board[0][4]), print_piece(self.board[1][4]),
                 print_piece(self.board[2][4]), print_piece(self.board[3][4]), print_piece(self.board[4][4]),
                 print_piece(self.board[5][4]), print_piece(self.board[6][4]));
        println!("{}  {}  {}  {}  {}  {}  {}", print_piece(self.board[0][3]), print_piece(self.board[1][3]),
                 print_piece(self.board[2][3]), print_piece(self.board[3][3]), print_piece(self.board[4][3]),
                 print_piece(self.board[5][3]), print_piece(self.board[6][3]));
        println!("{}  {}  {}  {}  {}  {}  {}", print_piece(self.board[0][2]), print_piece(self.board[1][2]),
                 print_piece(self.board[2][2]), print_piece(self.board[3][2]), print_piece(self.board[4][2]),
                 print_piece(self.board[5][2]), print_piece(self.board[6][2]));
        println!("{}  {}  {}  {}  {}  {}  {}", print_piece(self.board[0][1]), print_piece(self.board[1][1]),
                 print_piece(self.board[2][1]), print_piece(self.board[3][1]), print_piece(self.board[4][1]),
                 print_piece(self.board[5][1]), print_piece(self.board[6][1]));
        println!("{}  {}  {}  {}  {}  {}  {}", print_piece(self.board[0][0]), print_piece(self.board[1][0]),
                 print_piece(self.board[2][0]), print_piece(self.board[3][0]), print_piece(self.board[4][0]),
                 print_piece(self.board[5][0]), print_piece(self.board[6][0]));
        println!("____________________");
    }

    fn validmove(self, col: usize) -> (bool, usize)
    {
        if col >= 1 && col <= 7 {
            let firstvec = col - 1;

            for i in 0..6
                {
                    if self.board[firstvec][i].is_none()
                    {
                        return (true, i)
                    }
                };
        }
        (false, 10)
    }

    fn store_move(&mut self, col: usize, row: usize, player: Piece) {
        self.board[col][row] = Some(player);
    }
    fn check_win(&mut self, col: usize, row: usize, player: Piece) -> bool
    {
        let mut in_row = 1;
        if col != 0 {
            for i in (0..=col - 1).rev() //horizontal
                {
                    if self.board[i][row] == Some(player)
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
                    if self.board[i][row] == Some(player)
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
                    if self.board[col][i] == Some(player)
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
                    if self.board[col][i] == Some(player)
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
                    if self.board[adj_col][adj_row] == Some(player)
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
                    if self.board[adj_col][adj_row] == Some(player)
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
                        if self.board[adj_col][adj_row] == Some(player)
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
                        if self.board[adj_col][adj_row] == Some(player)
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
    fn board_not_full(&self) -> bool
    {
        for pie in self.board.iter()
            {
                for piec in pie.iter() {
                    if piec.is_none()
                    {
                        return true
                    }
                }
            }
        false
    }
    fn longest_row(&mut self, col: usize, row: usize, player: Piece) -> isize
    {
        let mut lengths = Vec::new();
        let mut in_row = 1;
        if col != 0 {
            for i in (0..=col - 1).rev() //horizontal
                {
                    if self.board[i][row] == Some(player)
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
                    if self.board[i][row] == Some(player)
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
            }
        lengths.push(in_row);
        in_row = 1;
        if row != 0 {
            for i in (0..=row - 1).rev() //vertical
                {
                    if self.board[col][i] == Some(player)
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
                    if self.board[col][i] == Some(player)
                    {
                        in_row += 1;
                    } else {
                        break;
                    }
                }
            }
        lengths.push(in_row);
        in_row = 1;
        for i in 1..5 //l diag
            {
                if row >= i && col >= i
                {
                    let adj_row = row - i;
                    let adj_col = col - i;
                    if self.board[adj_col][adj_row] == Some(player)
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
                    if self.board[adj_col][adj_row] == Some(player)
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
        lengths.push(in_row);
        in_row = 1;
        for i in 1..5 //r diag
            {
                if col >= i
                {
                    let adj_row = row + i;
                    let adj_col = col - i;

                    if adj_row <= 5
                    {
                        if self.board[adj_col][adj_row] == Some(player)
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
                        if self.board[adj_col][adj_row] == Some(player)
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
        lengths.push(in_row);

        *lengths.iter().max().unwrap()
    }

}

pub fn start_con(difficulty: usize)
{
    let mut new_game = ConGame::new();

    while new_game.winner == None && new_game.board_not_full() {
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
                new_game.store_move(col-1, row, Piece::X);
                if new_game.check_win(col-1, row, Piece::X)
                {
                    new_game.winner = Some(Piece::X);
                }
                else {
                     let next_move = new_game.minimax_search(difficulty * 6, true);
                     if let Some(m) = next_move {
                         let (val, loc) = new_game.clone().validmove(m + 1);
                         new_game.store_move(m, loc, Piece::O);
                         if new_game.check_win(m, loc, Piece::O) {
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

//---------------------------ConnectGame----------------------------------------------

#[cfg(test)]
mod con_tests {
    use super::Piece;
    use super::ConGame;
    use super::print_piece;

    #[test]
    fn new_con_test()
    {
        let con_1 = ConGame::new();
        assert_eq!(con_1.board[1][1], None);
    }

    #[test]
    fn store_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, Piece::X);
        assert_eq!(con_1.board[5][1].unwrap(), Piece::X);
    }

    #[test]
    fn print_piece_x_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, Piece::X);
        assert_eq!(print_piece(con_1.board[5][1]), "X");
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
    fn board_not_full_test()
    {
        let mut con_1 = ConGame::new();
        for a in 0..7
            {
                for b in 0..6
                    {
                        con_1.store_move(a, b, Piece::X);
                    }
            }
        assert_eq!(con_1.board_not_full(), false);
    }

    #[test]
    fn board_not_full2_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(5, 1, Piece::X);
        assert_eq!(con_1.board_not_full(), true);
    }

    #[test]
    fn lose_con_test()
    {
        let mut con_1 = ConGame::new();
        let a = con_1.check_win(5, 1, Piece::X);
        assert!(!a);
    }

    #[test]
    fn win_vert_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(1, 1,  Piece::X);
        con_1.store_move(1, 2,  Piece::X);
        con_1.store_move(1, 3, Piece::X);
        con_1.store_move(1, 4, Piece::X);
        let a = con_1.check_win(1, 4, Piece::X);
        assert!(a);
    }

    #[test]
    fn win_horiz_con_test()
    {
        let mut con_1 = ConGame::new();
        con_1.store_move(1, 1,  Piece::X);
        con_1.store_move(2, 1,  Piece::X);
        con_1.store_move(3, 1, Piece::X);
        con_1.store_move(4, 1, Piece::X);
        let a = con_1.check_win(4, 1, Piece::X);
        assert!(a);
    }
}
