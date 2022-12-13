use day13::{List, make, compare};

fn main() {
  let lines = global::read_strings();
  let packets: Vec<&String> = lines.iter().filter(|s| s.as_str() != "").collect();
  let mut lists: Vec<List> = packets.iter().map(|p| make(p)).collect();

  let dk1 = make(&"[[2]]".to_string());
  let dk2 = make(&"[[6]]".to_string());

  lists.push(dk1.to_vec());
  lists.push(dk2.to_vec());

  lists.sort_by(|l1,l2| compare(l1, l2));

  let mut result = 1;

  for i in 0..lists.len() {
    if lists[i] == dk1 { result = result * (i + 1) }
    if lists[i] == dk2 { result = result * (i + 1) }
  }

  println!("{:?}", result);
}