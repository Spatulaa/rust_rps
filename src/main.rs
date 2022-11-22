use rand::Rng;
use std::io;

enum RpsOption {
    Rock,
    Paper,
    Scissors,
    None,
}

impl RpsOption {
    fn to_string(&self) -> &str {
        match self {
            RpsOption::Rock => "rock",
            RpsOption::Paper => "paper",
            RpsOption::Scissors => "scissors",
            RpsOption::None => "none",
        }
    }
}

fn main() {
    println!("Rock, paper, or scissors?");

    // loops until user chooses a viable option
    let mut user_option = RpsOption::None;
    loop {
        match user_option {
            RpsOption::None => user_option = get_user_option(),
            _ => break,
        }
    }

    let bot_option = get_bot_option();
    find_winner(user_option, bot_option);
}

fn find_winner(first_pick: RpsOption, second_pick: RpsOption) {
    // finds winner by checking possible options
    let winner = match (&first_pick, &second_pick) {
        (RpsOption::Rock, RpsOption::Scissors) => 1,
        (RpsOption::Rock, RpsOption::Paper) => 2,
        (RpsOption::Paper, RpsOption::Rock) => 1,
        (RpsOption::Paper, RpsOption::Scissors) => 2,
        (RpsOption::Scissors, RpsOption::Rock) => 2,
        (RpsOption::Scissors, RpsOption::Paper) => 1,
        _ => 0,
    };

    // prints results
    match winner {
        1 => println!(
            "{} beats {} so player one wins!",
            first_pick.to_string(),
            second_pick.to_string()
        ),
        2 => println!(
            "{} beats {} so player one wins!",
            second_pick.to_string(),
            first_pick.to_string()
        ),
        _ => println!("you both chose {} so no one wins!", first_pick.to_string()),
    }
}

fn get_user_option() -> RpsOption {
    let mut option = String::new();

    // reads user input
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line :(");

    // matches input to avaiable options
    match &option.to_lowercase()[..option.len() - 1] {
        "rock" => RpsOption::Rock,
        "paper" => RpsOption::Paper,
        "scissors" => RpsOption::Scissors,
        _ => RpsOption::None,
    }
}

fn get_bot_option() -> RpsOption {
    // picks random option for bot
    match rand::thread_rng().gen_range(1..=3) {
        1 => RpsOption::Rock,
        2 => RpsOption::Paper,
        3 => RpsOption::Scissors,
        _ => RpsOption::None,
    }
}
