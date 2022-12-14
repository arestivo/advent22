use day5::{Ship, Move, CrateMover9001, MultipleMover};

fn main() {
  let lines = global::read_strings();

  let mut ship = Ship::build(&lines);
  let mut mover = CrateMover9001 {};

  let moves = Move::read(&lines);

  execute_moves(&mut ship, &mut mover, &moves);

  println!("{}", ship);
}

fn execute_moves(ship: &mut Ship, mover: &mut CrateMover9001, moves: &[Move]) {
  moves.iter().for_each(|m| mover.execute(m, ship));
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
  fn execute_moves_works() {
    let mut ship = make_ship();
    let mut mover = CrateMover9001 {};
    let moves = make_moves();

    execute_moves(&mut ship, &mut mover, &moves);

    let result = format!("{}", ship);

    assert_eq!("MCD", result)
  }
}