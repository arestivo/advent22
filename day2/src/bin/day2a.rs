use global;

fn main() {
  let total: i32 = global::read_strings()
    .iter()
    .map(|g| value(g).unwrap())
    .sum();

  println!("{total:?}")
}

fn value(game: &str) -> Result<i32, &str> {
  match game {
    "A X" => Ok(3 + 1),
    "A Y" => Ok(6 + 2),
    "A Z" => Ok(0 + 3),
    "B X" => Ok(0 + 1),
    "B Y" => Ok(3 + 2),
    "B Z" => Ok(6 + 3),
    "C X" => Ok(6 + 1),
    "C Y" => Ok(0 + 2),
    "C Z" => Ok(3 + 3),
    &_ => Err("Invalid Game"),
  }
}
