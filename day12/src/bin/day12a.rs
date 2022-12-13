fn main() {
  let lines = global::read_strings();
  let (heights, start, end) = day12::lines_to_heights(lines);

  let best = day12::fastest_path(&heights, &start, &end);

  println!("{:?}", best);
}