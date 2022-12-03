fn main() {
  let lines = global::read_strings();
  let mut iter = lines.iter();

  let mut total = 0;

  while iter.len() >= 3 {
    let (r1, r2, r3) = next_three(&mut iter);

    let common = r1.chars()
      .filter(|i| r2.contains(*i))
      .filter(|i| r3.contains(*i))
      .nth(0).unwrap();

    total += day3::priority(common);
  }

  println!("{total}");
}

fn next_three(iter: &mut std::slice::Iter<String>) -> (String, String, String) {
  (iter.next().unwrap().to_owned(), 
   iter.next().unwrap().to_owned(), 
   iter.next().unwrap().to_owned())
}