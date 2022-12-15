use regex::Regex;

#[derive(Debug)]
pub struct Position {
  pub x: i64,
  pub y: i64
}

#[derive(Debug)]
pub struct Segment {
  pub start: i64,
  pub end: i64
}

#[derive(Debug)]
pub struct SBPair {
  pub sensor: Position,
  pub beacon: Position
}

pub fn get_sensor_and_beacon(line: &str) -> (Position, Position) {
  let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
  let cap = re.captures_iter(line).next().unwrap();

  (
    Position { x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap() }, 
    Position { x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap() }
  )
}

pub fn starts_and_ends(segments: &[Segment]) -> (Vec<i64>, Vec<i64>) {
  let mut starts = segments.iter().map(|s| s.start).collect::<Vec<i64>>();
  starts.sort();
  starts.reverse();
  
  let mut ends = segments.iter().map(|s| s.end + 1).collect::<Vec<i64>>();
  ends.sort();
  ends.reverse();
  
  (starts, ends)
}

pub fn sb_to_segments(sb_pairs: &[SBPair], row: i64) -> Vec<Segment> {
  let mut segments = vec![];

  for sb in sb_pairs {
    let distance = sb.sensor.distance(&sb.beacon);
    let remaining = distance - (sb.sensor.y - row).abs();

    if remaining >= 0 {
      let start = sb.sensor.x - remaining;
      let end = sb.sensor.x + remaining;  
      segments.push(Segment {start, end});
    }
  }

  segments
}

impl Position {
  pub fn distance(&self, other: &Position) -> i64 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}