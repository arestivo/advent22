use global;

fn main() {
  let mut elfs: Vec<i32> = global::read_strings()
    .split(|l| l == "")
    .map(|e| e.iter().map(|s| s.parse::<i32>().unwrap()))
    .map(|v| v.sum::<i32>())
    .collect();

  elfs.sort();

  println!("{}", elfs.iter().rev().take(3).sum::<i32>())
}