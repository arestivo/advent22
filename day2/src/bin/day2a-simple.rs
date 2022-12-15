fn main() {
  let total: i32 = global::read_strings()
    .iter()
    .map(|g| value(g))
    .sum();

  println!("{total:?}")
}

fn value(game: &str) -> i32 {
  match game {
    "A X" => 3 + 1,
    "A Y" => 6 + 2,
    "A Z" =>     3,
    "B X" =>     1,
    "B Y" => 3 + 2,
    "B Z" => 6 + 3,
    "C X" => 6 + 1,
    "C Y" =>     2,
    "C Z" => 3 + 3,
    &_ => unreachable!()
  }
}