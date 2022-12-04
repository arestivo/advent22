use day4::{Assignment, filter_assignements};

fn main() {
  let lines = global::read_strings();
  let total = filter_assignements(lines, is_contained).len();

  println!("{total}")
}

fn is_contained(p: &Assignment) -> bool {
  p.0.0 <= p.1.0 && p.0.1 >= p.1.1 || p.1.0 <= p.0.0 && p.1.1 >= p.0.1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_contained_works() {
    assert!(is_contained(&((0, 1),(0, 1))));
    assert!(is_contained(&((1, 2),(0, 3))));
    assert!(is_contained(&((0, 3),(1, 2))));
    assert!(!is_contained(&((1, 2),(2, 3))));
  }
}