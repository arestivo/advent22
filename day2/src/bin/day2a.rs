use day2::GameResult;

fn main() {
  let total = day2::calculate_total(global::read_strings(), value_calculator);

  println!("{total}")
}

fn value_calculator(game: &str) -> i32 {
  let (h1, h2) = day2::normalize(game);

  match day2::result(&h1, &h2) {
    GameResult::LOSE => 0 + h2.value(),
    GameResult::DRAW => 3 + h2.value(),
    GameResult::WIN => 6 + h2.value(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_calculator_works() {
    assert_eq!(4, value_calculator("A X"));
    assert_eq!(8, value_calculator("A Y"));
    assert_eq!(3, value_calculator("A Z"));
    assert_eq!(1, value_calculator("B X"));
    assert_eq!(5, value_calculator("B Y"));
    assert_eq!(9, value_calculator("B Z"));
    assert_eq!(7, value_calculator("C X"));
    assert_eq!(2, value_calculator("C Y"));
    assert_eq!(6, value_calculator("C Z"));
  }
}