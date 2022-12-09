use std::collections::HashSet;

use day9::Point;

fn main() {
  let lines = global::read_strings();
  let moves = day9::get_moves(&lines);

  let mut rope = [Point::origin(); 10];

  let mut visited = HashSet::new();

  for m in moves {
    for _ in 0..m.num {
      rope[0].apply_move(&m);
      for i in 1..=9 {
        rope[i].follow(&rope[i-1].clone());
      }
      visited.insert(rope[9].clone());
    }
  }

  println!("{}", visited.len());
}