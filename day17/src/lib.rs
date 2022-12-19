use std::cmp::max;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Position {
  pub row: usize,
  pub col: usize
}

#[derive(Clone, Debug)]
pub struct Shape {
  pub id: usize,
  parts: Vec<Position>
}

#[derive(Clone, Debug)]
pub struct Rock {
  pub shape: Shape,
  pub pos: Position,
  pub landed: bool,
  pub top: usize
}

#[derive(Clone, Debug)]
pub struct Chamber {
  left: usize,
  right: usize,
  rocks: Vec<Rock>,
  pub top: usize
}

impl Chamber {
  pub fn new(left: usize, right: usize) -> Chamber { Chamber { left, right, rocks: vec![], top: 0 }}
  pub fn add(&mut self, rock: Rock) { self.rocks.push(rock.clone()); self.top = max(self.top, rock.top + 1) }

  fn valid(&self, to_test: &Rock) -> bool {
    for rock in self.rocks.clone() { 
      if to_test.pos.row > rock.top { continue; }
      if rock.intersects(to_test) { return  false }
    }
    
    true
  }

  pub fn draw(&self) {
    let top = self.top;
    let mut parts = vec![] ;
    for _ in 0..=top { parts.push(vec![false; 7]); }

    println!("TOP {} {}", top, parts.len());

    for r in self.rocks.clone() {
      for p in r.positions() {
        parts[p.row as usize][p.col as usize] = true
      }
    }

    for r in (0..=top).rev() {
      for c in 0..=6 {
        print!("{}", if parts[r][c] { "@" } else { "." });
      }
      println!();
    }
  }
}

impl Default for Chamber {
    fn default() -> Self {
        Self::new(0, 6)
    }
}

impl Shape {
  fn next(&self, shapes: &[Shape]) -> Shape {
    shapes[(self.id + 1) as usize % shapes.len()].clone()
  }

  fn new(id: usize, parts: &[(usize, usize)]) -> Shape { 
    Shape { id, parts: parts.iter().map(|(r, c)| Position {row: *r, col: *c}).collect() }
  }
}

impl Rock {
  pub fn new(shape: Shape, pos: Position) -> Rock {
    let mut top = 0;
    for part in shape.parts.clone() { top = max(top, part.row + pos.row) };
    
    Rock { shape, pos, landed: false, top }
  }

  pub fn first(chamber: &Chamber, shapes: &[Shape]) -> Rock {
    Rock::new(shapes[0].clone(), Position { row: chamber.top + 3, col: 2 })
  }

  pub fn next(&self, chamber: &Chamber, shapes: &[Shape]) -> Rock {
    Rock::new(self.shape.next(shapes), Position { row: chamber.top + 3, col: 2 })
  }

  pub fn left(&self, chamber: &Chamber) -> Rock {
    if self.pos.col == chamber.left { return self.clone(); }
    let rock = Rock::new(self.shape.clone(), self.pos.left());

    if chamber.valid(&rock) { rock } else { self.clone() }
  }

  pub fn right(&self, chamber: &Chamber) -> Rock {
    for p in self.positions() { if p.col == chamber.right { return self.clone(); }}
    let rock = Rock::new(self.shape.clone(), self.pos.right());

    if chamber.valid(&rock) { rock } else { self.clone() }
  }

  pub fn down(&self, chamber: &Chamber) -> Rock {
    let mut landed = self.clone(); landed.landed = true;
    if self.pos.row == 0 { return  landed ;}

    let rock = Rock::new(self.shape.clone(), self.pos.down());
    if chamber.valid(&rock) { rock } else {  landed }
  }

  pub fn is_at_bottom(&self) -> bool {
    for p in self.positions() {
      if p.row == 0 { return true; }
    }    

    false
  }

  fn positions(&self) -> Vec<Position> { 
    self.shape.parts
      .iter()
      .map(|p| Position{ row: p.row + self.pos.row, col: p.col + self.pos.col})
      .collect() 
  }

  fn intersects(&self, to_test: &Rock) -> bool {
    for p1 in self.positions() {
      for p2 in to_test.positions() {
        if p1 == p2 { return true }
      }      
    }

    false
  }

}

impl Position {
  fn left(&self) -> Position { Position { row: self.row, col: self.col - 1 }}
  fn right(&self) -> Position { Position { row: self.row, col: self.col + 1 }}
  fn down(&self) -> Position { Position { row: self.row - 1, col: self.col }}
}

pub fn create_rock_shapes() -> Vec<Shape> {
  let types = vec![
    Shape::new(0, &[(0, 0), (0, 1), (0, 2), (0, 3)]),
    Shape::new(1, &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
    Shape::new(2, &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
    Shape::new(3, &[(0, 0), (1, 0), (2, 0), (3, 0)]),
    Shape::new(4, &[(0, 0), (0, 1), (1, 0), (1, 1)])
  ];

  types
}