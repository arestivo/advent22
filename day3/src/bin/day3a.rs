fn main() {
  let lines = global::read_strings();

  let mut total = 0;

  for rucksack in lines {
    let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
    let common = c1.chars().filter(|i| c2.contains(*i)).nth(0).unwrap();

    total += day3::priority(common);
  }

  println!("{total}")
}

