use std::io::{stdin,stdout,Write};
use std::{process, env};

pub mod general_game;
pub mod tictactoe;
pub mod connect4;
pub mod checkers;

fn main() -> std::io::Result<()> {
    //let (game_num, difficulty_num) = choose_game(read_input().0, read_input().1);
    let game_num = choose_game(read_input().0);
    let difficulty_num = difficulty_level(read_input().1);

    general_game::lets_play(game_num, difficulty_num);
    Ok(())
}

/// Running each of the games as different programs:
/// cargo run tictactoe/ cargo run connect4/ cargo run checkers
///
/// There are 3 different difficulty levels: easy, medium, hard
/// User has option of setting difficulty level as second argument (e.g. cargo run checkers medium)
/// Otherwise, default is medium

fn read_input() -> (String, String) {
    let game = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error. Please provide a proper game name and run again");
        ::std::process::exit(1);
    });
    let difficulty = "".to_owned();
    if let Some(arg2) = env::args().nth(2) {
        let difficulty = arg2;
    }
    (game, difficulty)
}

fn choose_game(game: String) -> usize {
    //let difficulty_num = difficulty_level(difficulty);

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
    game_num
}

fn difficulty_level(difficulty: String) -> usize {
    let _ = stdout().flush();
    let difficulty = difficulty.to_lowercase();
    let difficulty_num =
        {
            if difficulty.contains("easy")
            {
                1
            } else if difficulty.contains("med")
            {
                2
            } else if difficulty.contains("hard")
            {
                3
            } else {
                2
            }
        };
    difficulty_num
}

#[cfg(test)]
mod start_tests {
    use super::*;

    #[test]
    fn difficulty_basic_easy() {
        assert_eq!(difficulty_level("easy".to_owned()), 1);
    }

    #[test]
    fn difficulty_basic_med() {
        assert_eq!(difficulty_level("medium".to_owned()), 2);
    }

    #[test]
    fn difficulty_basic_hard() {
        assert_eq!(difficulty_level("hard".to_owned()), 3);
    }

    #[test]
    fn difficulty_basic_no_input() {
        assert_eq!(difficulty_level("".to_owned()), 2);
    }

    #[test]
    fn difficulty_basic_wrong_input() {
        assert_eq!(difficulty_level("Bleh".to_owned()), 2);
    }

    #[test]
    fn difficulty_capital() {
        assert_eq!(difficulty_level("HARD".to_owned()), 3);
    }

    #[test]
    fn choose_game_tic(){
        assert_eq!(choose_game("tic".to_owned()), 1);
    }

    #[test]
    fn choose_game_con(){
        assert_eq!(choose_game("connect4".to_owned()), 2);
    }

    #[test]
    fn choose_game_check(){
        assert_eq!(choose_game("checkers".to_owned()), 3);
    }

    #[test]
    fn choose_game_check_cap(){
        assert_eq!(choose_game("CHECKERS".to_owned()), 3);
    }
}
fn choose_game2(game: String) -> usize {
    //let difficulty_num = difficulty_level(difficulty);

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
            eprintln!("Sorry, we do not offer that game! Please run the program again with either \
            tictactoe, checkers or connect4!");
            process::exit(1);
        }
    };
    game_num
}