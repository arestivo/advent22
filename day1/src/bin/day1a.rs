use global;

pub fn main() {
  let max = global::read_strings()
    .split(|l| l == "")
    .map(|e| e.iter().map(|s| s.parse::<i32>().unwrap()))
    .map(|v| v.sum::<i32>())
    .max();

  println!("{}", max.unwrap())
}
