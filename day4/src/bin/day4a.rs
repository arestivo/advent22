use day4::{Assignment, filter_assignements};

fn main() {
  let lines = global::read_strings();
  let total = filter_assignements(lines, contains).len();

  println!("{total}")
}

fn contains(p: &Assignment) -> bool {
  p.0.0 <= p.1.0 && p.0.1 >= p.1.1 || p.1.0 <= p.0.0 && p.1.1 >= p.0.1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn contains_works() {
    assert!(contains(&((0, 1),(0, 1))));
    assert!(contains(&((1, 2),(0, 3))));
    assert!(contains(&((0, 3),(1, 2))));
    assert!(!contains(&((1, 2),(2, 3))));
  }
}