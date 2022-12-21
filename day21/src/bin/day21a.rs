use std::collections::HashMap;

use day21::{lines_to_monkeys, Monkey, Operation, Term};

fn main() {
  let lines = global::read_strings();
  let monkeys = lines_to_monkeys(&lines);

  println!("{:?}", value(&monkeys, &"root".to_string()));
}

fn value(monkeys: &HashMap<String, Monkey>, word: &String) -> i64 {
  let monkey = monkeys.get(word).unwrap();
  match &monkey.term {
    Term::Number(n) => *n,
    Term::Calculation(left, op , right) => match op {
      Operation::Add => value(monkeys, left) + value(monkeys, right),
      Operation::Substract => value(monkeys, left) - value(monkeys, right),
      Operation::Multiply => value(monkeys, left) * value(monkeys, right),
      Operation::Divide => value(monkeys, left) / value(monkeys, right),
    }
  }
}