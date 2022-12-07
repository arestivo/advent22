fn main() {
  let lines = global::read_strings();
  
  let root = day7::read_folder(&lines);

  let mut total = 0;

  for folder in root {
    if folder.1 < 100000 { total += folder.1}
  }

  println!("{}", total);
}