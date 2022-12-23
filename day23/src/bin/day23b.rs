use std::collections::HashMap;

use day23::{Position};

fn main() {
  let lines = global::read_strings();
  let mut elves = day23::lines_to_elves(&lines);  
  let mut order = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

  let mut round = 0;

  loop {
    let map = day23::elves_to_map(&elves);
    let mut proposals: HashMap<Position, Vec<usize>> = HashMap::new();

    let mut moved = 0;

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
          if let Some(elf) = elves.get_mut(*i) { if elf.pos != p { moved += 1 }; elf.pos = p;  }
        }
      }
    }

    order.rotate_left(1);
    round += 1;

    if moved == 0 { println!("{}", round); break; }
  }
}