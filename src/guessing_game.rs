use rand::Rng;
use std::{cmp::Ordering, io};

const LOWER_BOUND: i32 = 1;
const UPPER_BOUND: i32 = 100;

pub fn guessing_game() {
  println!("Guess the number! [1-100]");

  let secret = rand::thread_rng().gen_range(LOWER_BOUND..UPPER_BOUND);

  let mut count = 0;
  while count <= 5 {
    count += 1;
    println!("input: ");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");

    println!("[{count}:] your guess: {guess}");

    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("only numbers");
        continue;
      }
    };

    match guess.cmp(&secret) {
      Ordering::Less => println!("too small"),
      Ordering::Greater => println!("too big"),
      Ordering::Equal => {
        println!("you win");
        break;
      }
    }
  }

  println!("secret: {secret}");
}
