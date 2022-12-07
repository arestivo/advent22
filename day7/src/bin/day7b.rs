fn main() {
  let lines = global::read_strings();
  
  let folders = day7::read_folders(&lines);

  let values = folders.into_values().collect::<Vec<u64>>();

  let total = values.iter().max().unwrap();
  let needed = 30000000 - (70000000 - total);
  let min = values.iter().filter(|v| *v >= &needed).min().unwrap();

  println!("{}", min);
}