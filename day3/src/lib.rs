pub fn priority(i: char) -> u32 {
  if i.is_ascii_lowercase() { i as u32 - 'a' as u32 + 1 } else { i as u32 - 'A' as u32 + 27 }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn value_calculator_works() {
    assert_eq!(1, priority('a'));
    assert_eq!(2, priority('b'));
    assert_eq!(26, priority('z'));
    assert_eq!(27, priority('A'));
    assert_eq!(28, priority('B'));
    assert_eq!(52, priority('Z'));
  }
}