use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::io;

fn get_computer_choice() -> i8 {
  let mut rng = ChaCha20Rng::from_entropy();
  rng.gen_range(0..99)
}

fn get_player_choice() -> i8 {
  println!("Rock, Paper, or Scissors?");
  print!("> ");
  io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  match input.trim().to_lowercase().as_str() {
    "rock" => 0,
    "r" => 0,
    "1" => 0,
    "paper" => 1,
    "p" => 1,
    "2" => 1,
    "scissors" => 2,
    "s" => 2,
    "3" => 2,
    _ => {
      println!("Invalid choice, please try again.");
      get_player_choice()
    }
  }
}

fn did_player_win(computer_choice: i8, player_choice: i8) -> i8 {
  let computer_choice_base = computer_choice % 3;

  // tie
  if computer_choice_base == player_choice {
    return 0;
  }

  // computer wins
  // rock > scissors
  if computer_choice_base == 0 && player_choice == 2 {
    return 1;
  }

  // computer wins
  // scissors > paper
  if computer_choice_base == 2 && player_choice == 1 {
    return 1;
  }

  // computer wins
  // paper > rock
  if computer_choice_base == 1 && player_choice == 0 {
    return 1;
  }

  // player wins
  return 2;
}

fn get_choice_name(choice: i8) -> String {
  match choice % 3 {
    0 => String::from("Rock"),
    1 => String::from("Paper"),
    2 => String::from("Scissors"),
    _ => String::from("Invalid"),
  }
}

fn main() {
  let computer_choice = get_computer_choice();
  let player_choice = get_player_choice();

  println!("Computer choice: {}", get_choice_name(computer_choice));

  if did_player_win(computer_choice, player_choice) == 2 {
    println!("You win!");
  } else if did_player_win(computer_choice, player_choice) == 1 {
    println!("You lose!");
  } else {
    println!("Tie!");
  }
}
