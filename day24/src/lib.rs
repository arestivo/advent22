use std::collections::HashSet;

use num::integer::lcm;
use priority_queue::PriorityQueue;

#[derive(Debug)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Position {
  pub row: isize,
  pub col: isize,
}

#[derive(Debug)]
pub struct Blizzard {
  pub start: Position,
  pub dir: Direction,
}

#[derive(Debug)]
pub struct Map {
  pub height: isize,
  pub width: isize,
  pub rows: Vec<Vec<Blizzard>>,
  pub cols: Vec<Vec<Blizzard>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State {
  pub pos: Position,
  pub step: isize,
  pub path: Vec<Position>
}

impl Blizzard {
  pub fn position(&self, map: &Map, step: isize) -> Position {
    match self.dir {
      Direction::Up => Position { row: (self.start.row - 1 - step).rem_euclid(map.height - 2) + 1, col: self.start.col },
      Direction::Down => Position { row: (self.start.row - 1 + step).rem_euclid(map.height - 2) + 1, col: self.start.col },
      Direction::Left => Position { row: self.start.row, col: (self.start.col - 1 - step).rem_euclid(map.width - 2) + 1 },
      Direction::Right => Position { row: self.start.row, col: (self.start.col - 1 + step).rem_euclid(map.width - 2) + 1 },
    }
  }
}

impl Position {
  pub fn is_blizzard(&self, map: &Map, step: isize) -> bool {
    if !self.is_inside(map) { return false }

    for b in &map.rows[self.row as usize] { if b.position(map, step as isize) == *self { return true } }
    for b in &map.cols[self.col as usize] { if b.position(map, step as isize) == *self { return true } }

    false
  }

  pub fn is_inside(&self, map: &Map) -> bool {
    !(self.row < 1 || self.col < 1 || self.row >= map.height - 1 || self.col >= map.width - 1)
  }

  pub fn distance(&self, other: &Position) -> isize {
    (self.row - other.row).abs() + (self.col - other.col).abs()
  }

  pub fn neighbours(&self) -> Vec<Position> {
    [(-1,0), (1,0), (0, -1), (0, 1)]
      .iter()
      .map(|(dr, dc)| Position { row: self.row + dr, col: self.col + dc })
      .collect()
  }
}

pub fn lines_to_map(lines: &[String]) -> Map {
  let mut map = Map { height: lines.len() as isize, width: lines[0].len() as isize, rows: vec![], cols: vec![] };

  for _ in 0..lines.len() { map.rows.push(vec![]); }
  for _ in 0..lines[0].len() { map.cols.push(vec![]); }

  for (row, line) in lines.iter().enumerate() {
    for (col, letter) in line.chars().enumerate() {
      match letter {
        '<' => map.rows[row].push(Blizzard { start: Position { row: row as isize, col: col as isize }, dir: Direction::Left }),
        '>' => map.rows[row].push(Blizzard { start: Position { row: row as isize, col: col as isize }, dir: Direction::Right }),
        'v' => map.cols[col].push(Blizzard { start: Position { row: row as isize, col: col as isize }, dir: Direction::Down }),
        '^' => map.cols[col].push(Blizzard { start: Position { row: row as isize, col: col as isize }, dir: Direction::Up }),
        _ => {}
      }
    } 
  }

  map
}

pub fn astar(map: &Map, start: Position, end: Position, first_step: isize) -> isize {
  let mut pq = PriorityQueue::new();
  let mut visited: HashSet<(Position, isize)> = HashSet::new();
  let mmc = lcm(map.height - 2, map.width - 2);

  pq.push(State { pos: start.clone(), step: first_step, path: vec![start] }, 0);

  while !pq.is_empty() {
    let (current, _) = pq.pop().unwrap();

    if visited.contains(&(current.pos.clone(), current.step % mmc)) { continue }
    visited.insert((current.pos.clone(), current.step % mmc));

    if !current.pos.is_blizzard(map, current.step + 1) {
      push_state(&mut pq, &current, &current.pos, &end);
    }

    for n in current.pos.neighbours() {
      if n == end { return current.step + 1 }
      else if n.is_inside(map) && !n.is_blizzard(map, current.step + 1) {
        push_state(&mut pq, &current, &n, &end);
      }
    }
  }

  -1
}

fn push_state(pq: &mut PriorityQueue<State, isize>, state: &State, pos: &Position, end: &Position) {
  let mut path = state.path.clone();
  path.push(pos.clone());
  let next = State { pos: pos.clone(), step: state.step + 1, path };
  let priority = -(state.step + 1 + next.pos.distance(end));
  pq.push(next, priority);
}