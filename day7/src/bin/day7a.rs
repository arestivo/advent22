fn main() {
  let lines = global::read_strings();
  
  let folders = day7::read_folders(&lines);

  let values = folders.into_values().collect::<Vec<u64>>();

  let sum = values.iter().filter(|v| *v <= &100000).sum::<u64>();

  println!("{}", sum);
}