use std::cmp::Ordering;

use day13::{make, compare};

fn main() {
  let lines = global::read_strings();
  let pairs:Vec<(String, String)> = lines.split(|l| l == "").map(|c| (c[0].to_owned(), c[1].to_owned())).collect();

  let mut index = 1;
  let mut sum = 0;

  for (s1, s2) in pairs {
    let (l1, l2) = ( make(&s1), make(&s2) );

    if compare(&l1, &l2) == Ordering::Less { sum += index }

    index += 1;
  }

  println!("{}", sum);
}