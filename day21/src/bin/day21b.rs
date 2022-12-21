use std::collections::HashMap;

use day21::{lines_to_monkeys, Monkey, Operation, Term};

fn main() {
  let lines = global::read_strings();
  let monkeys = lines_to_monkeys(&lines);
  let mut mem = HashMap::new();

  let root = monkeys.get(&"root".to_string()).unwrap();

  if let Term::Calculation(left, _, right) = &root.term {
    let left_value = value(&mut mem, &monkeys, left);
    let right_value = value(&mut mem, &monkeys, right);

    let humn = match left_value {
      Some(known) => match right_value {
          Some(_) => unreachable!("Nothing to do here"),
          None => equalize(&mut mem, &monkeys, known, monkeys.get(right).unwrap()),
      },
      None => match right_value {
          Some(known) => equalize(&mut mem, &monkeys, known, monkeys.get(left).unwrap()),
          None => unreachable!("Impossible to solve"),
      },
    };

    println!("{}", humn);
  }
}

fn equalize(mem: &mut HashMap<String, Option<i64>>, monkeys: &HashMap<String, Monkey>, known: i64, monkey: &Monkey) -> i64 {
  if monkey.word == "humn" { return known; }
  
  match &monkey.term {
    Term::Number(n) => if *n == known { unreachable!("Already there!") } else { unreachable!("{} != {}", known, n) },
    Term::Calculation(left, op, right) => {
      let left_value = value(mem, monkeys, left);
      let right_value = value(mem, monkeys, right);
      return match left_value {
        Some(left) => match right_value {
            Some(_) => unreachable!("Nothing to do here"),
            None => match op {
                Operation::Add => equalize(mem, monkeys, known - left, monkeys.get(right).unwrap()), 
                Operation::Substract => equalize(mem, monkeys, left - known, monkeys.get(right).unwrap()),
                Operation::Multiply => equalize(mem, monkeys, known / left, monkeys.get(right).unwrap()),
                Operation::Divide => equalize(mem, monkeys, known * left, monkeys.get(right).unwrap()),
            }
        },
        None => match right_value {
          Some(right) => match op {
            Operation::Add => equalize(mem, monkeys, known - right, monkeys.get(left).unwrap()), 
            Operation::Substract => equalize(mem, monkeys, known + right, monkeys.get(left).unwrap()),
            Operation::Multiply => equalize(mem, monkeys, known / right, monkeys.get(left).unwrap()),
            Operation::Divide => equalize(mem, monkeys, known * right, monkeys.get(left).unwrap()),
          }
          None => unreachable!("Impossible to solve"),
        },
      }
    }
  }
}

fn value(mem: &mut HashMap<String, Option<i64>>, monkeys: &HashMap<String, Monkey>, word: &String) -> Option<i64> {
  if word == "humn" { return None };
  if let Some(v) = mem.get(word) { return *v; }

  let monkey = monkeys.get(word).unwrap();
  let v = match &monkey.term {
    Term::Number(n) => Some(*n) ,
    Term::Calculation(left, op, right) => {
      if let Some(left) = value(mem, monkeys, left) {
        if let Some(right) = value(mem, monkeys, right) {
          match op {
            Operation::Add => Some(left + right),
            Operation::Substract => Some(left - right),
            Operation::Multiply => Some(left * right),
            Operation::Divide => Some(left / right),
          }    
        } else { None }
      } else { None }
    },
  };

  mem.insert(word.to_string(), v);
  
  v
}