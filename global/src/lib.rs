use std::{io, io::prelude::*};

pub fn read_strings() -> Vec<String> {
  io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

pub fn to_u32(s: &str) -> u32 {
  s.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn to_u32_works() {
    assert_eq!((1), to_u32("1"));
    assert_eq!((10), to_u32("10"));
    assert_eq!((123), to_u32("123"));
  }
}