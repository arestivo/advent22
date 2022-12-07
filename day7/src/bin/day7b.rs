fn main() {
  let lines = global::read_strings();
  
  let root = day7::read_folder(&lines);

  let values = root.into_values().collect::<Vec<u64>>();

  let total = values.iter().max().unwrap();
  let needed = 30000000 - (70000000 - total);
  let min = values.iter().filter(|v| *v >= &needed).min().unwrap();

  println!("{}", min);
}