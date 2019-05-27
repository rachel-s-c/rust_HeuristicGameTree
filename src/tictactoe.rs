use std::io::{stdin,stdout,Write};
use std::cmp::max;
use crate::general_game::{HeuristicGameTree, Piece};

const TICWINS: [[usize; 3]; 8] = [[0, 1, 2], [0, 3, 6], [0, 4, 8], [1, 4, 7], [2, 5, 8], [2, 4, 6], [3, 4, 5], [6, 7, 8]];


impl<'a> HeuristicGameTree for TicGame<'a> {
    type Move = usize;
    fn possible_moves(&mut self) -> Vec<Self::Move> {
        let mut list = Vec::new();
        for i in 0..9 {
            if self.board[i] == &Piece::None {
                list.push(i);
            }
        }
        list
    }
    // MAKE THIS BETTER
    fn heuristic(&self) -> isize {
        // invariant: x_streak != o_streak != 3
        // keep track of best streak by each player
        //
        let mut x_streak = if self.moves > 0 {1} else {0};
        let mut o_streak = if self.moves > 1 {1} else {0};
        // First check for wins
        if self.board[4] != &Piece::None {
            let center = self.board[4];
            if center == self.board[0] && center == self.board[8] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[1] && center == self.board[7] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[2] && center == self.board[6] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if center == self.board[3] && center == self.board[5] {
                if center == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        if self.board[0] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[1] && corner == self.board[2] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if corner == self.board[3] && corner == self.board[6] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        if self.board[8] != &Piece::None {
            let corner = self.board[8];
            if corner == self.board[5] && corner == self.board[2] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
            if corner == self.board[7] && corner == self.board[6] {
                if corner == &Piece::X {
                    x_streak = 3;
                }
                else {
                    o_streak = 3;
                }
            }
        }
        // Do pairs
        if self.board[4] != &Piece::None {
            let center = self.board[4];
            for i in 0..9 {
                if i != 4 && center == self.board[i] { // found a pair
                    if center == &Piece::X {
                        x_streak = max(x_streak,2);
                    }
                    else {
                        o_streak = max(o_streak,2);
                    }
                }
            }
        }
        if self.board[0] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[1] || corner == self.board[3] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[2] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[1] || corner == self.board[5] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[6] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[7] || corner == self.board[3] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        if self.board[8] != &Piece::None {
            let corner = self.board[0];
            if corner == self.board[7] || corner == self.board[5] {
                if corner == &Piece::X {
                    x_streak = max(x_streak,2);
                }
                else {
                    o_streak = max(o_streak,2);
                }
            }
        }
        x_streak - o_streak // Why is this backwards?
    }
    fn execute_move(&mut self, next_move: Self::Move, is_opponent: bool) {
        self.store_move(next_move, if is_opponent {&Piece::O} else {&Piece::X});
    }
}
//------------------------------------TicGame-----------------------------------------

#[derive (Clone)]
struct TicGame<'a>
{
    board: Vec<&'a Piece>,
    winner: Option<Piece>,
    moves: usize,
}

impl<'a> TicGame<'a>
{
    pub fn new() -> Self
    {

        TicGame {
            board: vec![&Piece::None; 9],
            winner: None,
            moves: 0,
        }
    }

    fn printboard(& mut self)
    {
        println!("  1  2  3");
        println!("A {}  {}  {}", self.board[0].val(), self.board[1].val(), self.board[2].val());
        println!("B {}  {}  {}", self.board[3].val(), self.board[4].val(), self.board[5].val());
        println!("C {}  {}  {}", self.board[6].val(), self.board[7].val(), self.board[8].val());
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

            if self.board[num] == &Piece::None {
                return (true, num)
            }
        }

        (false, 10)
    }

    fn store_move(&mut self, position:usize, player: &'a Piece){
        self.board[position] = player;
        self.moves += 1;
        if self.moves == 9
        {
            self.winner = Some(Piece::Tie);
        }
    }

    fn check_win(&mut self, player: Piece) -> bool
    {
        for vecs in TICWINS.iter()
            {
                let mut in_row = 0;
                for index in vecs.iter()
                    {
                        if self.board[*index] == &player
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

}

pub fn start_tic(difficulty: usize)
{
    let mut new_game = TicGame::new();

    while new_game.winner == None {
        println!("Where do you want to put your X? (row then column)");
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
                new_game.store_move(pos, &Piece::X);
                if new_game.check_win(Piece::X)
                {
                    new_game.winner = Some(Piece::X);
                } else {
                    let next_move = new_game.minimax_search(difficulty * 3, true);
                    if let Some(m) = next_move {
                        new_game.store_move(m, &Piece::O);
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
    println!("{} WON THE GAME!", new_game.winner.unwrap().val());
}

//------------------------------------TicGame-----------------------------------------
