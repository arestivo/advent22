use std::{collections::HashSet, cmp::{min, max}};

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Position {
  r: u32,
  c: u32
}

impl Position {
  fn down(&self) -> Position {
    Position { r: self.r + 1, c: self.c }
  }

  fn left(&self) -> Position {
    Position { r: self.r + 1, c: self.c - 1 }
  }

  fn right(&self) -> Position {
    Position { r: self.r + 1, c: self.c + 1 }
  }
}

#[derive(Debug)]
pub struct Cave {
  pub walls: HashSet<Position>,
  pub sand: HashSet<Position>  
}

impl Cave {
  pub fn new() -> Cave {
    Cave { walls: HashSet::new(), sand: HashSet::new() }
  }

  pub fn add_path(&mut self, path: &String) {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut last: Option<Position> = None;

    for point in re.captures_iter(path) {
      let p = Position { r: point[2].parse().unwrap(), c: point[1].parse().unwrap() };

      match last {
        Some(l) => {
          if l.r == p.r { for c in min(l.c,p.c)..=max(l.c,p.c) { self.walls.insert(Position {r: l.r, c}); } }
          if l.c == p.c { for r in min(l.r,p.r)..=max(l.r,p.r) { self.walls.insert(Position {r, c: l.c}); } }
        }
        None => {}
      }

      last = Some(p);
    }
  }

  pub fn empty(&self, p: &Position) -> bool {
    !self.walls.contains(p) && !self.sand.contains(p)
  }

  pub fn add_sand_a(&mut self) -> bool {
    let mut position = Position { r: 0, c: 500 };
    let bottom = self.walls.iter().map(|w| w.r).max().unwrap();

    loop {
      if position.r > bottom { return false; }

      if self.empty(&position.down()) { position = position.down(); }
      else if self.empty(&position.left()) { position = position.left(); }
      else if self.empty(&position.right()) { position = position.right(); }
      else { self.sand.insert(position); break; }
    }

    true
  }

  pub fn add_sand_b(&mut self) -> bool {
    let mut position = Position { r: 0, c: 500 };
    let bottom = self.walls.iter().map(|w| w.r).max().unwrap();

    loop {
      if position.r + 1 == bottom + 2 { self.sand.insert(position.clone()); break; }
      else if self.empty(&position.down()) { position = position.down(); }
      else if self.empty(&position.left()) { position = position.left(); }
      else if self.empty(&position.right()) { position = position.right(); }
      else { self.sand.insert(position.clone()); break; }
    }

    position.r != 0
  }
}