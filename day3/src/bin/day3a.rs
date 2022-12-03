fn main() {
  let lines = global::read_strings();
  let total = lines.iter().fold(0, |acc, r| acc + day3::priority(common(r)));
  println!("{total}")
}

fn common(rucksack: &String) -> char {
  let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
  c1.chars().filter(|i| c2.contains(*i)).nth(0).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_calculator_works() {
    assert_eq!('p', common(&String::from("vJrwpWtwJgWrhcsFMMfFFhFp")));
    assert_eq!('L', common(&String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")));
    assert_eq!('P', common(&String::from("PmmdzqPrVvPwwTWBwg")));
    assert_eq!('v', common(&String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")));
    assert_eq!('t', common(&String::from("ttgJtRGJQctTZtZT")));
    assert_eq!('s', common(&String::from("CrZsJsPPZsGzwwsLwLmpwMDw")));
  }
}