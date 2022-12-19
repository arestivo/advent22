use regex::Regex;

#[derive(Debug,PartialEq, Eq, Hash, Clone)]
pub struct Position {
  pub x: i64,
  pub y: i64,
  pub z: i64
}

impl Position {
  pub fn neighbours(&self) -> Vec<Position>{
    vec![
      Position { x: self.x + 1, y: self.y, z: self.z },
      Position { x: self.x - 1, y: self.y, z: self.z },
      Position { x: self.x, y: self.y + 1, z: self.z },
      Position { x: self.x, y: self.y - 1, z: self.z },
      Position { x: self.x, y: self.y, z: self.z + 1 },
      Position { x: self.x, y: self.y, z: self.z - 1 },
    ]
  }
}

pub fn lines_to_cubes(lines: &[String]) -> Vec<Position> {
  let mut cubes = vec![];
  let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
  for line in lines {
    let cap = re.captures_iter(line).next().unwrap();
    cubes.push(Position { x: cap[1].parse::<i64>().unwrap(), y: cap[2].parse::<i64>().unwrap(), z: cap[3].parse::<i64>().unwrap() })
  }
  cubes
}