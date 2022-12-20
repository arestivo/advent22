use day20::{mix, sum_grooves};

fn main() {
  let lines = global::read_strings();
  let numbers: Vec<i64> = lines.iter().map(|l| l.parse::<i64>().unwrap() * 811589153).collect();
  let numbers = mix(&numbers, 10);

  println!("{:?}", sum_grooves(&numbers));
}