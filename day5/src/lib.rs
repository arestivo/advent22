use std::collections::{VecDeque};
use std::fmt;
use regex::Regex;

pub enum Model { CrateMover9000, CrateMover9001 }

pub struct CrateMover9000 { }
pub struct CrateMover9001 { }

impl SingleMover for CrateMover9000 {}
impl MultipleMover for CrateMover9001 {}

pub struct Ship {
  stacks: Vec<VecDeque<char>>
}

pub struct Move {
  from: usize,
  to: usize,
  qty: u32
}

impl Ship {
  pub fn build(lines: &Vec<String>) -> Ship {
    let mut ship = Ship { stacks: vec![] };

    for _ in (1..=lines[0].len() + 1).step_by(4) {
      ship.stacks.push(VecDeque::new())
    }

    for line in lines.iter() {
      if !line.contains("[") { break };
      for c in (1..=line.len()).step_by(4) {
        let letter = line.chars().nth(c).unwrap();
        if letter != ' ' {
          ship.stacks[(c - 1) / 4].push_front(letter)
        }
      }
    }

    ship
  }  
}

pub trait SingleMover {
  fn execute(&mut self, m: &Move, s: &mut Ship) {
    for _ in 0..m.qty {
      let c: char = s.stacks[m.from].pop_back().unwrap();
      s.stacks[m.to].push_back(c);
    }
  }
}

pub trait MultipleMover {
  fn execute(&mut self, m: &Move, s: &mut Ship) {
    let mut crates = VecDeque::new();
    for _ in 0..m.qty {
      let c: char = s.stacks[m.from].pop_back().unwrap();
      crates.push_front(c);
    }

    for c in crates {
      s.stacks[m.to].push_back(c);
    }
  }
}

impl Move {
  pub fn read(lines: &Vec<String>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines.iter() {
      for g in re.captures_iter(line) {
          moves.push(Move {
            from: g[2].parse::<usize>().unwrap() - 1, 
            to: g[3].parse::<usize>().unwrap() - 1, 
            qty: g[1].parse::<u32>().unwrap()
          })
      } 
    };

    moves
  }
}

impl fmt::Display for Ship {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v: Vec<&char> = self.stacks.iter().map(|s| s.back().unwrap()).collect();
    let s: String = v.into_iter().collect();
    write!(f, "{}", s)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  fn make_input() -> Vec<String> {
    vec![
      "    [D]    ".to_string(),
      "[N] [C]    ".to_string(),
      "[Z] [M] [P]".to_string(),
      " 1   2   3 ".to_string(),
      "".to_string(),
      "move 1 from 2 to 1".to_string(),
      "move 3 from 1 to 3".to_string(),
      "move 2 from 2 to 1".to_string(),
      "move 1 from 1 to 2".to_string(),
    ]
  }

  fn make_ship() -> Ship {
    Ship::build(&make_input())
  }

  fn make_moves() -> Vec<Move> {
    Move::read(&make_input())
  }

  #[test]
  fn ship_build_works() {
    let ship = make_ship();

    assert_eq!(3, ship.stacks.len());

    assert_eq!(2, ship.stacks[0].len());
    assert_eq!(3, ship.stacks[1].len());
    assert_eq!(1, ship.stacks[2].len());

    assert_eq!(&'D', ship.stacks[1].back().unwrap());
    assert_eq!(&'M', ship.stacks[1].front().unwrap());
  }

  #[test]
  fn read_move_works() {
    let moves = make_moves();

    assert_eq!(4, moves.len());
    assert_eq!(1, moves[0].from);
    assert_eq!(2, moves[1].to);
    assert_eq!(2, moves[2].qty);
    
  }

  #[test]
  fn crate_mover_9000_works() {
    let mut ship = make_ship();
    let mut mover = CrateMover9000 {};

    mover.execute(&Move{from: 0, to: 1, qty: 2}, &mut ship);

    assert_eq!(&'Z', ship.stacks[1].back().unwrap());
    assert_eq!(&'M', ship.stacks[1].front().unwrap());
    assert_eq!(0, ship.stacks[0].len());
  }

  #[test]
  fn crate_mover_9001_works() {
    let mut ship = make_ship();
    let mut mover = CrateMover9001 {};

    mover.execute(&Move{from: 0, to: 1, qty: 2}, &mut ship);

    assert_eq!(&'N', ship.stacks[1].back().unwrap());
    assert_eq!(&'M', ship.stacks[1].front().unwrap());
    assert_eq!(0, ship.stacks[0].len());
  }
}