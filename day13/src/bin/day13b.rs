use day13::{List, make, compare};

fn main() {
  let lines = global::read_strings();
  let packets: Vec<&String> = lines.iter().filter(|s| s.as_str() != "").collect();
  let mut lists: Vec<List> = packets.iter().map(|p| make(p)).collect();

  let dk1 = make(&"[[2]]".to_string());
  let dk2 = make(&"[[6]]".to_string());

  lists.push(dk1.to_vec());
  lists.push(dk2.to_vec());

  lists.sort_by(compare);

  let mut result = 1;

  for (i, list) in lists.iter().enumerate() {
    if list == &dk1 { result *= i + 1 }
    if list == &dk2 { result *= i + 1 }
  }

  println!("{:?}", result);
}