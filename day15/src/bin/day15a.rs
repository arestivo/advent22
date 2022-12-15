use std::collections::HashSet;

use day15::{get_sensor_and_beacon, SBPair, starts_and_ends, sb_to_segments};

fn main() {
  let lines = global::read_strings();
  let mut sb_pairs = vec![];

  for line in lines.clone() {
    let (sensor, beacon) = get_sensor_and_beacon(&line);
    sb_pairs.push(SBPair {sensor, beacon});
  }

  println!("{:?}", impossible(&sb_pairs, if lines.len() > 14 { 2000000 } else { 10 }));
}

fn impossible(sb_pairs: &[SBPair], row: i64) -> i64 {
  let segments = sb_to_segments(sb_pairs, row);
  let (mut starts, mut ends) = starts_and_ends(&segments);

  let mut current = i64::MIN;
  let mut active = 0;
  let mut total = 0;
  
  while !starts.is_empty() || !ends.is_empty() {
    if !starts.is_empty() && starts[starts.len() - 1] < ends[ends.len() - 1] {
      let s = starts.pop().unwrap();
      active += 1;
      if active == 1 { current = s }
    } else {
      let e = ends.pop().unwrap();
      active -= 1;
      if active == 0 { total += e - current }
    }
  }

  let existing: HashSet<i64> = HashSet::from_iter(sb_pairs.iter().filter(|p| p.beacon.y == row).map(|p| p.beacon.x));

  total - existing.len() as i64
}