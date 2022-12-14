use day14::Cave;

fn main() {
  let paths = global::read_strings();
  let mut cave = Cave::new();

  for path in paths { cave.add_path(&path) }
  while cave.add_sand_b() {}

  println!("{}", cave.sand.len());
}