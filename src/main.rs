mod guessing_game;
mod temperature;
mod fibonacci;
mod factorial;

fn main() {
  if false {
    guessing_game::guessing_game();
  }

  println!("100 f == {} c", temperature::fahrenheit_to_celcius(100.0));
  println!("100 c == {} f", temperature::celcius_to_fahrenheit(100.0));

  println!("fibonacci(7) == {}", fibonacci::fibonacci(7));
  println!("7! == {}", factorial::factorial(7));
}
