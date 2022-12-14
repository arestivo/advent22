use std::collections::HashSet;

use day9::Point;

fn main() {
  let lines = global::read_strings();
  let moves = day9::get_moves(&lines);

  let mut head = Point::origin();
  let mut tail = Point::origin();

  let mut visited = HashSet::new();

  for m in moves {
    for _ in 0..m.num {
      head.apply_move(&m);
      tail.follow(&head);
      visited.insert(tail);
    }
  }

  println!("{}", visited.len());
}