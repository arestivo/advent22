#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
  pub dir: Direction,
  pub num: u32
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Point {
  pub row: i32,
  pub col: i32
}

pub fn get_moves(lines: &Vec<String>) -> Vec<Move> {
  let mut moves = vec![];

  let re = regex::Regex::new(r"^([UDLR]) (\d+)$").unwrap();

  for line in lines {
    let cm = re.captures_iter(line).next().unwrap();
    let (ch, num) = (cm[1].to_string(), cm[2].to_string().parse().unwrap());
    let m = match ch.as_str() {
      "U" => Move { dir: Direction::Up, num },
      "D" => Move { dir: Direction::Down, num },
      "L" => Move { dir: Direction::Left, num },
      "R" => Move { dir: Direction::Right, num },
      &_ => unreachable!()
    };
    moves.push(m);
  }

  moves
}

impl Point {
  pub fn origin() -> Point {
    Point { row: 0, col: 0 }
  }

  pub fn apply_move(&mut self, m: &Move){
    match m.dir {
      Direction::Up => { self.row -= 1 },
      Direction::Down => { self.row += 1 },
      Direction::Left => { self.col -= 1 },
      Direction::Right => { self.col += 1 },
    };  
  }

  fn direction(p1: &Point, p2: &Point) -> (Option<Move>, Option<Move>) {
    let mut m1 = None;
    let mut m2 = None;

    if p1.row - p2.row > 1 { m1 = Some(Move{dir: Direction::Up, num: 1}) }
    else if p1.row - p2.row < -1 { m1 = Some(Move{dir: Direction::Down, num: 1}) }
    else if p1.row - p2.row == 1 && (p1.col - p2.col).abs() > 1 { m1 = Some(Move{dir: Direction::Up, num: 1}) }
    else if p1.row - p2.row == -1 && (p1.col - p2.col).abs() > 1 { m1 = Some(Move{dir: Direction::Down, num: 1}) }

    if p1.col - p2.col > 1 { m2 = Some(Move{dir: Direction::Left, num: 1}) }
    else if p1.col - p2.col < -1 { m2 = Some(Move{dir: Direction::Right, num: 1}) }
    else if p1.col - p2.col == 1 && (p1.row - p2.row).abs() > 1 { m2 = Some(Move{dir: Direction::Left, num: 1}) }
    else if p1.col - p2.col == -1 && (p1.row - p2.row).abs() > 1 { m2 = Some(Move{dir: Direction::Right, num: 1}) }
    
    (m1, m2)
  }

  pub fn follow(&mut self, other: &Point) {
    let (m1, m2) = Point::direction(self, other);

    if let Some(m1) = m1 { self.apply_move(&m1); }
    if let Some(m2) = m2 { self.apply_move(&m2); }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn apply_move_works() {
    let mut p = Point{row: 0, col: 0};
    p.apply_move(&(Move {dir: Direction::Up, num: 1}));
    assert_eq!(Point{row: -1, col: 0}, p);

    let mut p = Point{row: 0, col: 0};
    p.apply_move(&(Move {dir: Direction::Down, num: 1}));
    assert_eq!(Point{row: 1, col: 0}, p);

    let mut p = Point{row: 0, col: 0};
    p.apply_move(&(Move {dir: Direction::Left, num: 1}));
    assert_eq!(Point{row: 0, col: -1}, p);

    let mut p = Point{row: 0, col: 0};
    p.apply_move(&(Move {dir: Direction::Right, num: 1}));
    assert_eq!(Point{row: 0, col: 1}, p);
  }

  #[test]
  fn direction_works() {
    let p1 = Point {row: 0, col: 0};

    let p2 = Point {row: 1, col: 0};
    assert_eq!((None, None), Point::direction(&p1, &p2));

    let p2 = Point {row: 2, col: 0};
    assert_eq!((Some(Move {dir: Direction::Down, num: 1}), None), Point::direction(&p1, &p2));

    let p2 = Point {row: -2, col: 0};
    assert_eq!((Some(Move {dir: Direction::Up, num: 1}), None), Point::direction(&p1, &p2));

    let p2 = Point {row: 0, col: 2};
    assert_eq!((None, Some(Move {dir: Direction::Right, num: 1})), Point::direction(&p1, &p2));

    let p2 = Point {row: 0, col: -2};
    assert_eq!((None, Some(Move {dir: Direction::Left, num: 1})), Point::direction(&p1, &p2));

    let p2 = Point {row: 2, col: 1};
    assert_eq!((Some(Move {dir: Direction::Down, num: 1}), Some(Move {dir: Direction::Right, num: 1})), Point::direction(&p1, &p2));
  }

  #[test]
  fn follow_works() {
    let mut p1 = Point {row: 0, col: 0};
    let p2 = Point {row: 2, col: 1};

    p1.follow(&p2);

    assert_eq!(Point {row: 1, col: 1}, p1);
  }
}