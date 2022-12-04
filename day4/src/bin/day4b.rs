use day4::{Assignment, filter_assignements};

fn main() {
  let lines = global::read_strings();
  let total = filter_assignements(lines, overlaps).len();

  println!("{total}")
}

fn overlaps(p: &Assignment) -> bool {
  p.0.0 <= p.1.1 && p.1.0 <= p.0.1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn overlaps_works() {
    assert!(overlaps(&((0, 1),(0, 1))));
    assert!(overlaps(&((1, 2),(0, 3))));
    assert!(overlaps(&((0, 3),(1, 2))));
    assert!(overlaps(&((1, 2),(2, 3))));
    assert!(!overlaps(&((1, 2),(3, 4))));
  }
}