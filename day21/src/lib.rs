use std::{collections::HashMap};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
pub enum Operation { 
  #[display("+")]
  Add, 
  #[display("-")]
  Substract, 
  #[display("*")]
  Multiply, 
  #[display("/")]
  Divide 
}

#[derive(Display, FromStr, Debug, Clone)]
pub enum Term {
  #[display("{0}")]
  Number(i64),
  #[display("{0} {1} {2}")]
  Calculation(String, Operation, String)
}

#[derive(Display, FromStr, Debug, Clone)]
#[display("{word}: {term}")]
pub struct Monkey {
  pub word: String,
  pub term: Term
}

pub fn lines_to_monkeys(lines: &[String]) -> HashMap<String, Monkey> {
  lines
    .iter()
    .map(|s| s.parse::<Monkey>())
    .map(Result::unwrap)
    .map(|m: Monkey| (m.clone().word, m))
    .collect::<HashMap<_, _>>()
}