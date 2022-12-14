use day2::GameHand;

fn main() {
  let total = day2::calculate_total(global::read_strings(), value_calculator);

  println!("{total}")
}

fn value_calculator(game: &str) -> i32 {
  let (h1, h2) = day2::normalize(game);

  match h2 {
    GameHand::ROCK => 0 + h1.worse().value(),
    GameHand::PAPER => 3 + h1.value(),
    GameHand::SCISSOR => 6 + h1.better().value(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_calculator_works() {
    assert_eq!(3, value_calculator("A X"));
    assert_eq!(4, value_calculator("A Y"));
    assert_eq!(8, value_calculator("A Z"));
    assert_eq!(1, value_calculator("B X"));
    assert_eq!(5, value_calculator("B Y"));
    assert_eq!(9, value_calculator("B Z"));
    assert_eq!(2, value_calculator("C X"));
    assert_eq!(6, value_calculator("C Y"));
    assert_eq!(7, value_calculator("C Z"));
  }
}