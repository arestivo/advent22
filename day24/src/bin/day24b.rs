use day24::{lines_to_map, Position, astar};

fn main() {
  let lines = global::read_strings();
  let map = lines_to_map(&lines);

  let there = astar(&map, Position {row: 0, col: 1}, Position {row: map.height - 1, col: map.width - 2}, 0);
  let back = astar(&map, Position {row: map.height - 1, col: map.width - 2}, Position {row: 0, col: 1}, there);
  let again = astar(&map, Position {row: 0, col: 1}, Position {row: map.height - 1, col: map.width - 2}, back);

  println!("{}", again);
}