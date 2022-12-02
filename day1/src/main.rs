use global;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 { panic!("Missing Part!") }

  let problem = args[1].as_str();

  match problem {
    "a" => parta(),
    "b" => partb(),
    _ => println!("Unknown Part!")
  }
}

fn parta() {
  let max = global::read_strings()
    .split(|l| l == "")
    .map(|e| e.iter().map(|s| s.parse::<i32>().unwrap()))
    .map(|v| v.sum::<i32>())
    .max();
  
  println!("{}", max.unwrap())
}

fn partb() {
  let mut elfs: Vec<i32> = global::read_strings()
    .split(|l| l == "")
    .map(|e| e.iter().map(|s| s.parse::<i32>().unwrap()))
    .map(|v| v.sum::<i32>())
    .collect();

  elfs.sort();

  println!("{}", elfs.iter().rev().take(3).sum::<i32>())
}