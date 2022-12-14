use regex::Regex;

#[derive(Debug,Clone)]
pub enum Value {
  Old,
  Number(i64)
}

#[derive(Debug,Clone)]
pub enum Op {
  Times, Plus
}

type Operation = (Value, Op, Value);

#[derive(Debug,Clone)]
pub struct Monkey {
  pub items: Vec<i64>,
  pub operation: Operation,
  pub divisible: i64,
  pub iftrue: usize,
  pub iffalse: usize,
  pub inspections: i64
}

impl Monkey {
  fn new(lines: &[String]) -> Monkey {

    let mut items = vec![];
    let re = Regex::new(r"(\d+)").unwrap();
    for c in re.captures_iter(&lines[1]) {
      items.push(c.get(1).unwrap().as_str().parse().unwrap());
    }

    let re = Regex::new(r"(\d+|old) (\+|\*) (\d+|old)").unwrap();
    let c = re.captures_iter(&lines[2]).next().unwrap();
    
    let operation = (
      if c[1].to_string() == "old" { Value::Old } else { Value::Number(c[1].to_string().parse().unwrap()) },
      match c[2].to_string().as_str() { "+" => { Op::Plus } "*" => { Op::Times } &_ => panic!("Unknown operator")},
      if c[3].to_string() == "old" { Value::Old } else { Value::Number(c[3].to_string().parse().unwrap()) }
    );

    let divisible = global::extract_number_from_string::<i64>(&lines[3]);
    let iftrue = global::extract_number_from_string::<usize>(&lines[4]) as usize;
    let iffalse = global::extract_number_from_string::<usize>(&lines[5]) as usize;

    Monkey { items, operation, divisible, iftrue, iffalse, inspections: i64::from(0) }
  }

  pub fn execute_operation(op: Operation, value: i64) -> i64 {
    let v1 = match op.0 { Value::Old => { value } Value::Number(n) => { n } };
    let v2 = match op.2 { Value::Old => { value } Value::Number(n) => { n } };

    match op.1 { Op::Plus => { v1 + v2 } Op::Times => {v1 * v2} }
  }

}

pub fn lines_to_monkeys(lines: &[String]) -> Vec<Monkey> {
  let mut monkeys = vec![];
  let chunks:Vec<&[String]> = lines.split(|l| l.is_empty()).collect();

  for chunk in chunks {
    monkeys.push(Monkey::new(chunk));
  }

  monkeys
}