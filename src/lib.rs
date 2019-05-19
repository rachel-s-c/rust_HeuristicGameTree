use std::io::{stdin,stdout,Write};


const ticwins: [[usize; 3]; 8] = [[0, 1, 2], [0, 3, 6], [0, 4, 8], [1, 4, 7], [2, 5, 8], [2, 4, 6], [3, 4, 5], [6, 7, 8]];
const rows: [&str; 3] = ["A", "B", "C"];


pub fn lets_play(game: usize, diff: usize)
{
    match game
        {
            1 => starttic(),
            2 => println!("connect4"),
            3 => println!("checkers"),
            _ => println!("error"),
        }
}
#[derive (PartialEq, Clone)]
pub enum Piece {
    X,
    O,
    None,
    Tie
}

impl Piece {
    fn is_X(&self) -> bool {
        if let Piece::X = self {
            true
        }
        else {false}
    }
    fn is_O(&self) -> bool {
        if let Piece::O = self {
            true
        }
        else {false}
    }
    fn is_None(&self) -> bool {
        if let Piece::None = self {
            true
        }
        else {false}
    }
    fn val(&self) -> &str
    {
        if self.is_X()
        {
         return "X"
        }
        if self.is_O()
        {
            return "O"
        }
        if self.is_None()
        {
            return ""
        }
        "Tie"
    }
}

#[derive (Clone)]
struct ticgame<'a>
{
    board: Vec<&'a Piece>,
    winner: Option<Piece>,
    moves: usize,
}

impl<'a> ticgame<'a>
{
    fn new() -> Self
    {

        ticgame {
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
            let int = match row
                {
                    "A" => 0,
                    "B" => 3,
                    "C" => 6,
                    _ => 100,
                };

            if self.board[int + col - 1] == &Piece::None
            {
                return (true, int+col - 1)
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

    fn check_win(& mut self, player: Piece) -> bool
    {
        for vecs in ticwins.iter()
            {
                let mut in_row = 0;

                for index in vecs.iter()
                    {
                        if self.board[index.clone()] == &player
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




        /*
        let mut X = Vec::new();
        let mut O = Vec::new();

        for (index, symbol) in self.board.iter().enumerate() //starts at 1
        {
            if symbol == &"X"
            {
                X.push(index);
            }

            if symbol == &"O"
            {
                O.push(index);
            }
        }
        for vecs in ticwins
            {
                if X.contains(vecs[0]) & X.contains(vecs[1]) & X.contains(vecs[2]){
                    return (true, "X")
                }

                if O.contains(vecs[0]) & O.contains(vecs[1]) & O.contains(vecs[2]){
                    return (true, "O")
                }
            }
        (false, "")
    }
    */

}

fn starttic()
{
    let mut new_game = ticgame::new();

    while new_game.winner == None {
        println!("Where do you want to put your X? (row then column)");
        new_game.printboard();
        let mut loc = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut loc).expect("Did not enter a correct string");
        let mut loc = loc.split_whitespace();
        let row = match loc.next()
            {
                Some(a) => a,
                None => "", //Throw error, input wrong
            };
        let col = match loc.next()
            {
                Some(a) => a,
                None => "", //Throw error, input wrong
            };
        if loc.next() == None
        {
           let (valid, pos) = new_game.clone().validmove(row, col.parse().unwrap()); //throw error
            if valid
            {
                new_game.store_move(pos, &Piece::X);
                if new_game.check_win(Piece::X)
                {
                    new_game.winner = Some(Piece::X);
                }
            }
            // BASIC O PLAYER
            if valid
            {
                let mut cont = true;
                for row in rows.iter() {
                    for col in 1..3
                        {
                            let (valid2, pos2) = new_game.clone().validmove(*row, col);
                            if valid2 & cont
                            {
                                new_game.store_move(pos2, &Piece::O);
                                if new_game.check_win(Piece::O)
                                {
                                    new_game.winner = Some(Piece::O);
                                }
                                cont = false;
                            }
                        }
                }
            }
        }
       // else  throw error ASK FOR NEW INPUT

    }
    println!("{} WON THE GAME!", new_game.winner.unwrap().val())

}

