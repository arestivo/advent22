#[derive(PartialEq, Eq)]
pub enum GameResult { WIN, DRAW, LOSE }
#[derive(PartialEq, Eq)]
pub enum GameHand { ROCK, PAPER, SCISSOR }

impl GameHand {
  pub fn value(&self) -> i32 {
    match self {
      GameHand::ROCK => 1,
      GameHand::PAPER => 2,
      GameHand::SCISSOR => 3
    }
  }

  pub fn worse(&self) -> GameHand {
    match self {
      GameHand::ROCK => GameHand::SCISSOR,
      GameHand::PAPER => GameHand::ROCK,
      GameHand::SCISSOR => GameHand::PAPER
    }
  }

  pub fn better(&self) -> GameHand {
    match self {
      GameHand::ROCK => GameHand::PAPER,
      GameHand::PAPER => GameHand::SCISSOR,
      GameHand::SCISSOR => GameHand::ROCK
    }
  }
}

pub fn calculate_total(values: Vec<String>, value_calculator: fn(&str) -> i32) -> i32 {
  values
    .iter()
    .map(|g| value_calculator(g))
    .sum()
}

pub fn normalize(game: &str) -> (GameHand, GameHand) {
  let (p1, p2) = game.split_once(" ").unwrap();

  let p1 = match p1 { "A" => GameHand::ROCK, "B" => GameHand::PAPER, "C" => GameHand::SCISSOR, &_ => unreachable!() };
  let p2 = match p2 { "X" => GameHand::ROCK, "Y" => GameHand::PAPER, "Z" => GameHand::SCISSOR, &_ => unreachable!() };

  (p1, p2)
}

pub fn result(h1: &GameHand, h2: &GameHand) -> GameResult {
  if &h1.better() == h2 { GameResult::WIN }
  else if &h2.better() == h1 { GameResult::LOSE }
  else { GameResult::DRAW }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_works() {
    assert_eq!(GameHand::ROCK.value(), 1);
    assert_eq!(GameHand::PAPER.value(), 2);
    assert_eq!(GameHand::SCISSOR.value(), 3);
  }

  #[test]
  fn result_works() {
    assert!(matches!(result(&GameHand::ROCK, &GameHand::ROCK), GameResult::DRAW));
    assert!(matches!(result(&GameHand::ROCK, &GameHand::PAPER), GameResult::WIN));
  }

  #[test]
  fn normalize_works() {
    assert!(matches!(normalize("A X"), (GameHand::ROCK, GameHand::ROCK)));
    assert!(matches!(normalize("B Y"), (GameHand::PAPER, GameHand::PAPER)));
    assert!(matches!(normalize("C Z"), (GameHand::SCISSOR, GameHand::SCISSOR)));
  }
}