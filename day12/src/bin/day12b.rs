use day12::Position;

fn main() {
  let lines = global::read_strings();
  let (heights, _, end) = day12::lines_to_heights(lines);

  let mut best = u32::MAX;

  for r in 0..heights.len() {
    for c in 0..heights[0].len() {
      if heights[r][c] == 0 {
        let current = day12::fastest_path(&heights, &Position{ r, c }, &end);
        if current < best { best = current }
      }
    }
  }

  println!("{:?}", best);
}