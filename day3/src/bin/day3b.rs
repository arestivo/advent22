use day3::priority;

fn main() {
  let total: u32 = global::read_strings()
    .chunks(3)
    .map(common)
    .map(priority)
    .sum();

  println!("{}", total);
}

fn common(r: &[String]) -> char {
  r[0].chars()
    .filter(|i| r[1].contains(*i)).find(|i| r[2].contains(*i)).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn common_works() {
    assert_eq!('r', common(&[
      String::from("vJrwpWtwJgWrhcsFMMfFFhFp"), 
      String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 
      String::from("PmmdzqPrVvPwwTWBwg")
    ]));
    assert_eq!('Z', common(&[
      String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 
      String::from("ttgJtRGJQctTZtZT"), 
      String::from("CrZsJsPPZsGzwwsLwLmpwMDw")
    ]));
  }
}