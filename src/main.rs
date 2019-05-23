use std::io::{stdin,stdout,Write};
use std::process;
use finalproject::lets_play;


fn main() -> std::io::Result<()> {
    println!("Welcome to Gamer's Paradise! What would you like to play: Tic-Tac-Toe, Connect4, or Checkers?");
    let mut game= String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut game).expect("Did not enter a correct string");
    let game = game.to_lowercase();
    let game_num = {
        if game.contains("tic")
        {
            1
        } else if game.contains("connect4")
        {
            2
        } else if game.contains("checkers")
        {
            3
        } else {
            eprintln!("Sorry, we do not offer that game! BYE!");
            process::exit(1);
        }
    };
    println!("What difficulty would you like to play? Easy, medium, or hard");
    let mut difficulty= String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut difficulty).expect("Did not enter a correct string");
    let difficulty = difficulty.to_lowercase();
    let difficulty_num=
    {
        if difficulty.contains("easy")
        {
            0
        } else if difficulty.contains("medium")
        {
            1
        } else if difficulty.contains("hard")
        {
            2
        } else {
            eprintln!("Sorry, we do not offer that difficulty! BYE!");
            process::exit(1);
        }
    };

    lets_play(game_num, difficulty_num);

    Ok(())
}
