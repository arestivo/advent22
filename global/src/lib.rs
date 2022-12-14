use std::{io, io::prelude::*, str::FromStr};
use regex::Regex;

pub fn read_strings() -> Vec<String> {
  io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

pub fn read_single_line() -> String {
  io::stdin().lock().lines().next().unwrap().unwrap()
}

pub fn lines_to_array(lines: Vec<String>) -> Vec<Vec<u32>> {
  let mut trees = vec![];

  for line in lines {
    let mut row = vec![];
    for c in line.chars() {
      row.push(c.to_digit(10).unwrap());
    }
    trees.push(row);
  }

  trees
}

pub fn extract_number_from_string<T: FromStr>(s: &String) -> T {
  let re = Regex::new(r"(\d+)").unwrap();
  let c = re.captures_iter(s).next().unwrap();
  match c[1].to_string().parse::<T>() {
    Ok(v) => v,
    Err(_) => panic!("Failed to parse number from {}", s)
  }
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