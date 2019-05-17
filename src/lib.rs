
const ticwins: [[usize; 3]; 8] = [[1, 2, 3], [1, 4, 7], [1, 5, 9], [2, 5, 8], [3, 6, 9], [3, 5, 7], [4, 5, 6], [7, 8, 9]];


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

pub enum Piece {
    X,
    O,
    None
}

impl Piece {
    fn is_X(&self) -> bool {
        if let Piece::X = self {
            true
        }
        else {false}
    }
    fn is_Y(&self) -> bool {
        if let Piece::Y = self {
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
}

struct ticgame<'a>
{
    board: Vec<&'a Piece>,
    winner: Option<Piece>,
}

impl<'a> ticgame<'a>
{
    fn new() -> Self
    {

        ticgame {
            board: vec![None; 9],
            winner: None,
        }
    }

    fn printboard(& mut self)
    {
        println!("  1  2  3");
        println!("A {}  {}  {}", self.board[0], self.board[1], self.board[2]);
        println!("B {}  {}  {}", self.board[3], self.board[4], self.board[5]);
        println!("C {}  {}  {}", self.board[6], self.board[7], self.board[8]);
    }

    fn validmove(self, row: &'a str, col: usize) -> (bool, usize)
    {
        if row == "A" || row == "B" || row == "C"
        {
            if col == 1 || col == 2 || col == 3
            {
                let int = match row
                    {
                        "A" => 0,
                        "B" => 3,
                        "C" => 6,
                        _ => 100,
                    };

                if self.board[int + col] == None
                {
                    (true, int+col)
                }
            }
        }

        (false, 10)
    }

    fn store_move(&mut self, position:usize, ){
        //self.board[position] =
    }

    fn check_win(& mut self) -> (bool, &str)
    {
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

                if O.contains(vecs[0]) & X.contains(vecs[1]) & X.contains(vecs[2]){
                    return (true, "O")
                }
            }
        (false, "")
    }
}

fn starttic()
{
    let mut new_game = ticgame::new();
    new_game.printboard();
    println!("Where do you want to put your X? (row then column)")

}

