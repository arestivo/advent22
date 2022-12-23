use day22::{lines_to_map, lines_to_path, Position};

fn main() {
  let lines = global::read_strings();
  let map = lines_to_map(&lines);
  let path = lines_to_path(&lines);

  let mut p = Position { row: 0, col: 0, dir: day22::Direction::Right };

  while p.tile(&map) == ' ' { p = Position {col: p.col + 1, ..p}}

  for i in path {
    println!("{:?} {}", p, i);

    match i {
      day22::Instruction::Left => p = p.turn_left(),
      day22::Instruction::Right => p = p.turn_right(),
      day22::Instruction::Walk(d) => p = p.walk(d, &map),
    }
  }

  println!("{:?}", p);

  println!("{:?}", (p.row + 1) * 1000 + (p.col + 1) * 4 + (p.dir as isize));
}