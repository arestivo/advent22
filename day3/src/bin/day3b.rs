use day3::priority;

fn main() {
  let lines = global::read_strings();

  let x: Vec<u32> = lines
    .chunks(3)
    .map(|g| common(g))
    .map(|c| priority(c))
    .collect();

  println!("{}", x.iter().sum::<u32>());
}

fn common(r: &[String]) -> char {
  r[0].chars()
    .filter(|i| r[1].contains(*i))
    .filter(|i| r[2].contains(*i))
    .nth(0).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_calculator_works() {
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