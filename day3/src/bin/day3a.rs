fn main() {
  let total = global::read_strings()
    .iter()
    .fold(0, |acc, r| acc + day3::priority(common(r)));

  println!("{total}")
}

fn common(rucksack: &String) -> char {
  let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
  c1.chars().find(|i| c2.contains(*i)).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn common_works() {
    assert_eq!('p', common(&String::from("vJrwpWtwJgWrhcsFMMfFFhFp")));
    assert_eq!('L', common(&String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")));
    assert_eq!('P', common(&String::from("PmmdzqPrVvPwwTWBwg")));
    assert_eq!('v', common(&String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")));
    assert_eq!('t', common(&String::from("ttgJtRGJQctTZtZT")));
    assert_eq!('s', common(&String::from("CrZsJsPPZsGzwwsLwLmpwMDw")));
  }
}