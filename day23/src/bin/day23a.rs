use std::collections::HashMap;

use day23::{Position};

fn main() {
  let lines = global::read_strings();
  let mut elves = day23::lines_to_elves(&lines);  
  let mut order = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

  for _ in 0..10 {
    let map = day23::elves_to_map(&elves);
    let mut proposals: HashMap<Position, Vec<usize>> = HashMap::new();

    for (i, elf) in elves.iter().enumerate() {
      let p = elf.propose(&map, &order);
      if proposals.contains_key(&p) { 
        let mut v = proposals.get(&p).unwrap().to_owned();
        v.push(i);
        proposals.insert(p.clone(),  v); 
      }
      else { proposals.insert(p.clone(), vec![i]); }
    }

    for (p, v) in proposals {
      if v.len() == 1 {
        if let Some(i) = v.first() {
          if let Some(elf) = elves.get_mut(*i) { elf.pos = p }
        }
      }
    }

    order.rotate_left(1);
  }

  let map = day23::elves_to_map(&elves);

  let min_row = map.iter().map(|p| p.row).min().unwrap();
  let max_row = map.iter().map(|p| p.row).max().unwrap();
  let min_col = map.iter().map(|p| p.col).min().unwrap();
  let max_col = map.iter().map(|p| p.col).max().unwrap();

  let empty = (max_row - min_row + 1) * (max_col - min_col + 1) - elves.len() as i64;

  println!("{:?}", empty);
}