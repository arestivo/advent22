use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Elf {
  pub pos : Position
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Position {
  pub row: i64,
  pub col: i64  
}

pub type Map = HashSet<Position>;

pub fn lines_to_elves(lines: &[String]) -> Vec<Elf> {
  let mut elves = vec![];

  for (row, line) in lines.iter().enumerate() {
    for (col, value) in line.chars().enumerate() {
      if value == '#' { elves.push(Elf { pos: Position { row: row as i64, col: col as i64 } })}
    }
  }

  elves
}

pub fn elves_to_map(elves: &[Elf]) -> Map {
  let mut map = HashSet::new();

  for elf in elves {
    map.insert(elf.pos.clone());
  }

  map
}

pub fn print_map(map: &Map) {
  let min_row = map.iter().map(|p| p.row).min().unwrap();
  let max_row = map.iter().map(|p| p.row).max().unwrap();
  let min_col = map.iter().map(|p| p.col).min().unwrap();
  let max_col = map.iter().map(|p| p.col).max().unwrap();

  for row in min_row..=max_row {
    for col in min_col..=max_col {
      print!("{}", if map.contains(&Position {row, col}) { "#" } else { "." });
    }    
    println!();
  }
}

impl Elf {
  pub fn propose(&self, map: &Map, order: &[(i64, i64)]) -> Position {
    let mut count = 0;

    for dr in -1..=1 {
      for dc in -1..=1 {
        if (dr != 0 || dc != 0) && map.contains(&Position { row: self.pos.row + dr, col: self.pos.col + dc }) { count += 1; }
      }
    }

    if count > 0 {
      for (dr, dc) in order {
        if self.check(map, dr, dc) { return Position { row: self.pos.row + dr, col: self.pos.col + dc }}
      }
    }

    self.pos.clone()
  }

  pub fn check(&self, map: &Map, dr: &i64, dc: &i64) -> bool {
    if *dc == 0 { 
      !map.contains(&Position { row: self.pos.row + dr, col: self.pos.col - 1 }) &&
      !map.contains(&Position { row: self.pos.row + dr, col: self.pos.col }) &&
      !map.contains(&Position { row: self.pos.row + dr, col: self.pos.col + 1 })    
    } else if *dr == 0 {
      !map.contains(&Position { row: self.pos.row - 1, col: self.pos.col + dc }) &&
      !map.contains(&Position { row: self.pos.row, col: self.pos.col + dc }) &&
      !map.contains(&Position { row: self.pos.row + 1, col: self.pos.col + dc })    
    } else {
      unreachable!()
    }
  }
}