use std::cmp::{max, min};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
pub enum Instruction {
  #[display("L")]
  Left,
  #[display("R")]
  Right,
  #[display("{0}")]
  Walk(usize)  
}

#[derive(Clone, Debug)]
pub enum Direction {
  Right = 0,
  Down = 1,
  Left = 2,
  Up = 3
}

#[derive(Clone, Debug)]
pub struct Position {
  pub row: isize,
  pub col: isize,
  pub dir: Direction
}

#[derive(Clone, Debug)]
pub struct CubePosition {
  pub pos: Position,
  pub face: u8
}

impl Position {
  pub fn turn_left(&self) -> Position {
    Position { dir: match self.dir {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
      }, 
    ..*self }
  }

  pub fn turn_right(&self) -> Position {
    Position { dir: match self.dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
      }, 
    ..*self }
  }

  pub fn rotate_right(&self, size: isize) -> Position {
    Position {
      row: self.col,
      col: size - self.row - 1,
      ..self.turn_right()
    }
  }

  pub fn rotate_left(&self, size: isize) -> Position {
    Position {
      row: size - self.col - 1,
      col: self.row,
      ..self.turn_left()
    }
  }

  pub fn invert(&self, size: isize) -> Position {
    self.rotate_left(size).rotate_left(size)
  }

  fn width(map: &[Vec<char>]) -> isize { map[0].len() as isize }
  fn height(map: &[Vec<char>]) -> isize { map.len() as isize }
  
  pub fn tile(&self, map: &[Vec<char>]) -> char { map [self.row as usize][self.col as usize] }

  fn next_no_wrap(&self) -> Position {
    match self.dir {
        Direction::Up => Position { row: self.row - 1, ..self.clone()},
        Direction::Right => Position { col: self.col + 1, ..self.clone()},
        Direction::Down => Position { row: self.row + 1, ..self.clone()},
        Direction::Left => Position { col: self.col - 1, ..self.clone()},
    }
  }

  fn next_wrap(&self, map: &[Vec<char>]) -> Position {
    let n = self.next_no_wrap();
    if n.row < 0 { return Position { row: Position::height(map) - 1, ..n } };
    if n.row >= Position::height(map) { return Position { row: 0, ..n } };
    if n.col < 0 { return Position { col: Position::width(map) - 1, ..n } };
    if n.col >= Position::width(map) { return Position { col: 0, ..n } };
    n
  }

  fn next(&self, map: &[Vec<char>]) -> Position {
    let mut n = self.next_wrap(map);

    while n.tile(map) == ' ' { n = n.next_wrap(map) }

    n
  }

  pub fn walk (&self, distance: usize, map: &[Vec<char>]) -> Position {
    let mut p = self.clone();
    let mut n = self.clone();

    for _ in 0..distance { n = n.next(map); if n.tile(map) == '#' { return p }; p = n.clone(); }

    n
  }
}

impl CubePosition {
  pub fn turn_left(&self) -> CubePosition {
    CubePosition { pos: self.pos.turn_left(), face: self.face }
  }
  
  pub fn turn_right(&self) -> CubePosition {
    CubePosition { pos: self.pos.turn_right(), face: self.face }
  }

  fn face_size(map: &[Vec<char>]) -> isize { 
    min(Position::height(map), Position::width(map)) / 3 
  }
  
  pub fn map_position(&self, map: &[Vec<char>]) -> Position { 
    let size = CubePosition::face_size(map);

    match self.face {
      1 => Position { row: self.pos.row as isize, col: (size + self.pos.col) as isize, dir: self.pos.dir.clone() }, 
      2 => Position { row: self.pos.row as isize, col: (size * 2 + self.pos.col) as isize, dir: self.pos.dir.clone() },
      3 => Position { row: (size + self.pos.row) as isize, col: (size + self.pos.col) as isize, dir: self.pos.dir.clone() }, 
      4 => Position { row: (size * 2 + self.pos.row) as isize, col: self.pos.col as isize, dir: self.pos.dir.clone() }, 
      5 => Position { row: (size * 2 + self.pos.row) as isize, col: size + self.pos.col as isize, dir: self.pos.dir.clone() }, 
      6 => Position { row: (size * 3 + self.pos.row) as isize, col: self.pos.col as isize, dir: self.pos.dir.clone() }, 
      _ => unreachable!()
    }
  }

  pub fn tile(&self, map: &[Vec<char>]) -> char { 
    let p = self.map_position(map);

    map[p.row as usize][p.col as usize]
  }

  fn next_no_wrap(&self) -> CubePosition {
    CubePosition { pos: self.pos.next_no_wrap(), face: self.face }    
  }

  fn next_face(&self) -> u8 {
    match self.face {
      1 => match self.pos.dir { Direction::Right => 2, Direction::Down => 3, Direction::Left => 4, Direction::Up => 6 }
      2 => match self.pos.dir { Direction::Right => 5, Direction::Down => 3, Direction::Left => 1, Direction::Up => 6 }
      3 => match self.pos.dir { Direction::Right => 2, Direction::Down => 5, Direction::Left => 4, Direction::Up => 1 }
      4 => match self.pos.dir { Direction::Right => 5, Direction::Down => 6, Direction::Left => 1, Direction::Up => 3 }
      5 => match self.pos.dir { Direction::Right => 2, Direction::Down => 6, Direction::Left => 4, Direction::Up => 3 }
      6 => match self.pos.dir { Direction::Right => 5, Direction::Down => 2, Direction::Left => 1, Direction::Up => 4 }
      _ => unreachable!()
    }
  }

  fn next_pos(&self, next_face: u8, size: isize) -> Position {
    let p = Position { row: self.pos.row.rem_euclid(size), col: self.pos.col.rem_euclid(size), dir: self.pos.dir.clone() };

    if self.face == 1 && next_face == 2 { return p}
    if self.face == 1 && next_face == 3 { return p }
    if self.face == 1 && next_face == 4 { return p.invert(size) }
    if self.face == 1 && next_face == 6 { return p.rotate_right(size) }

    if self.face == 2 && next_face == 5 { return p.invert(size) }
    if self.face == 2 && next_face == 3 { return p.rotate_right(size) }
    if self.face == 2 && next_face == 1 { return p }
    if self.face == 2 && next_face == 6 { return p }

    if self.face == 3 && next_face == 2 { return p.rotate_left(size) }
    if self.face == 3 && next_face == 5 { return p }
    if self.face == 3 && next_face == 4 { return p.rotate_left(size) }
    if self.face == 3 && next_face == 1 { return p }

    if self.face == 4 && next_face == 5 { return p }
    if self.face == 4 && next_face == 6 { return p }
    if self.face == 4 && next_face == 1 { return p.invert(size) }
    if self.face == 4 && next_face == 3 { return p.rotate_right(size) }

    if self.face == 5 && next_face == 2 { return p.invert(size)}
    if self.face == 5 && next_face == 6 { return p.rotate_right(size) }
    if self.face == 5 && next_face == 4 { return p }
    if self.face == 5 && next_face == 3 { return p }

    if self.face == 6 && next_face == 5 { return p.rotate_left(size) }
    if self.face == 6 && next_face == 2 { return p }
    if self.face == 6 && next_face == 1 { return p.rotate_left(size) }
    if self.face == 6 && next_face == 4 { return p }

    unreachable!("{} -> {}", self.face, next_face)
  }

  fn next_wrap(&self, map: &[Vec<char>]) -> CubePosition {
    let size = CubePosition::face_size(map);
    let n = self.next_no_wrap();
    let nf = self.next_face();

    if n.pos.row < 0 || n.pos.col < 0 || n.pos.row >= size || n.pos.col >= size { 
      return CubePosition{ pos: n.next_pos(nf, size), face: nf } 
    }

    n
  }

  pub fn walk (&self, distance: usize, map: &[Vec<char>]) -> CubePosition {
    let mut p = self.clone();
    let mut n = self.clone();

    for _ in 0..distance { n = n.next_wrap(map); if n.tile(map) == '#' { return p }; p = n.clone(); }

    n
  }
}

pub fn lines_to_map(lines: &[String]) -> Vec<Vec<char>> {
  let width = lines[0..lines.len()-2].iter().fold(0, |acc, line| max(acc, line.len()));
  let mut map = vec![vec![' '; width]; lines.len() - 2];

  for r in 0..lines.len() - 2 {
    for c in 0..width {
      map[r][c] = if c >= lines[r].len() { ' ' } else { lines[r].chars().nth(c).unwrap() };
    }
  }

  map
}

pub fn lines_to_path(lines: &[String]) -> Vec<Instruction> {
  lines[lines.len() - 1]
    .replace('R', " R ")
    .replace('L', " L ")
    .split(' ')
    .map(|s| s.parse::<Instruction>())
    .map(Result::unwrap)
    .collect()
}