fn main() {
  let lines = global::read_strings();
  
  let root = day7::read_folder(&lines);

  let values = root.into_values().collect::<Vec<u64>>();

  let sum = values.iter().filter(|v| *v <= &100000).sum::<u64>();

  println!("{}", sum);
}