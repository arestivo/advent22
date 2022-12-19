use std::collections::HashSet;

use day18::Position;

fn main() {
  let lines = global::read_strings();
  let cubes = day18::lines_to_cubes(&lines);
  let exists: HashSet<&Position> = HashSet::from_iter(cubes.iter());

  println!("{:?}", cubes
      .iter()
      .map(|c| c.neighbours().iter().filter(|s| !exists.contains(s)).count())
      .sum::<usize>());
}