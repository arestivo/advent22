use day15::{get_sensor_and_beacon, SBPair, starts_and_ends, sb_to_segments};

fn main() {
  let lines = global::read_strings();
  let mut sb_pairs = vec![];

  for line in lines.clone() {
    let (sensor, beacon) = get_sensor_and_beacon(&line);
    sb_pairs.push(SBPair {sensor, beacon});
  }

  for y in 0..(if lines.len() > 14 { 4000000 } else { 20 }) {
    if let Some(x) = possible(&sb_pairs, y) { println!("{}", x * 4000000 + y); return }
  }
}

fn possible(sb_pairs: &[SBPair], row: i64) -> Option<i64> {
  let segments = sb_to_segments(sb_pairs, row);
  let (mut starts, mut ends) = starts_and_ends(&segments);

  let mut current = i64::MIN;
  let mut active = 0;

  while !starts.is_empty() || !ends.is_empty() {
    if !starts.is_empty() && starts[starts.len() - 1] < ends[ends.len() - 1] {
      let s = starts.pop().unwrap();
      active += 1;
      if active == 1 && current != i64::MIN && s - current == 1 { return Some(current) }
    } else {
      let e = ends.pop().unwrap();
      active -= 1;
      if active == 0 { current = e }
    }
  }

  None
}