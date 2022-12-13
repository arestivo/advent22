use std::cmp::{self, Ordering};

pub type List = Vec<Element>;

#[derive(Debug,Eq,PartialEq,Clone)]
pub enum Element {
  Number(u32),
  List(List)
}

#[derive(Debug,Eq,PartialEq)]
enum Token {
  Open,
  Close,
  Number(u32),
  Comma
}

pub fn make(s: &String) -> List {
  let mut tokens = tokenize(s);
  parse(&mut tokens)
}

pub fn compare(l1: &List, l2: &List) -> cmp::Ordering {
  let length = cmp::min(l1.len(), l2.len());

  for i in 0..length {
    match (&l1[i], &l2[i]) {
      (Element::Number(n1), Element::Number(n2)) => {
        if n1 < &n2 { return Ordering::Less }
        if n1 > &n2 { return Ordering::Greater }
      }
      (Element::Number(n1), Element::List(l2)) => {
        let r = compare(&vec![Element::Number(*n1)], &l2);
        if r != Ordering::Equal { return r }
      }
      (Element::List(l1), Element::Number(n2)) => {
        let r = compare(&l1, &vec![Element::Number(*n2)]);
        if r != Ordering::Equal { return r }
      }
      (Element::List(l1),Element::List (l2)) => { 
        let r = compare(&l1, &l2);
        if r != Ordering::Equal { return r }
      }
    }
  }

  if l1.len() < l2.len() { return Ordering::Less }
  if l1.len() > l2.len() { return Ordering::Greater }

  Ordering::Equal
}

fn parse(tokens: &mut Vec<Token>) -> List {
  let mut l: List = vec![];

  if *tokens.last().unwrap() != Token::Open { panic!("Bad Stack") }
  tokens.pop();

  while !tokens.is_empty() {
    let t = tokens.last().unwrap();

    match t {
      Token::Number(n) => { l.push(Element::Number(*n)) }
      Token::Open => { l.push(Element::List(parse(tokens))) }
      Token::Close => { return l }
      Token::Comma => { }
    }

    tokens.pop();
  }

  panic!("Bad Stack")
}

fn tokenize(s: &String) -> Vec<Token> {
  let mut stack = vec![];
  let mut digits = "".to_string();

  for i in 0..s.len() {
    let c = s.chars().nth(i).unwrap();

    if digits != "" && !c.is_ascii_digit() {
      stack.push(Token::Number(digits.parse().unwrap()));
      digits = "".to_string();
    }
    
    match c {
      '[' => { stack.push(Token::Open) },
      ']' => { stack.push(Token::Close) },
      ',' => { stack.push(Token::Comma) },
      n if n.is_ascii_digit() => { digits.push(n) },
      _ => panic!("Failed parsing")
    }
  }

  stack.reverse();

  stack
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tokenizer_works() {
    assert_eq!(vec![Token::Close, Token::Open], tokenize(&"[]".to_string()));
    assert_eq!(vec![Token::Close, Token::Number(123), Token::Open], tokenize(&"[123]".to_string()));
    assert_eq!(vec![
      Token::Close, 
        Token::Number(3), Token::Comma, 
        Token::Number(2), Token::Comma, 
        Token::Number(1), 
      Token::Open], tokenize(&"[1,2,3]".to_string()));
  }

  #[test]
  fn parse_works() {
    assert_eq!(vec![] as List, parse(&mut tokenize(&"[]".to_string())));
    assert_eq!(vec![Element::Number(123)] as List, parse(&mut tokenize(&"[123]".to_string())));
    assert_eq!(vec![Element::Number(1),
      Element::Number(2),
      Element::Number(3)
    ] as List, parse(&mut tokenize(&"[1,2,3]".to_string())));
    assert_eq!(vec![Element::Number(1),
      Element::List(vec![Element::Number(2)]),
      Element::List(vec![Element::Number(3),Element::Number(4)])
    ] as List, parse(&mut tokenize(&"[1,[2],[3,4]]".to_string())));
  }

  #[test]
  fn make_works() {
    assert_eq!(vec![] as List, make(&"[]".to_string()));
    assert_eq!(vec![Element::Number(123)] as List, make(&"[123]".to_string()));
    assert_eq!(vec![Element::Number(1),
      Element::Number(2),
      Element::Number(3)
    ] as List, make(&"[1,2,3]".to_string()));
    assert_eq!(vec![Element::Number(1),
      Element::List(vec![Element::Number(2)]),
      Element::List(vec![Element::Number(3),Element::Number(4)])
    ] as List, make(&"[1,[2],[3,4]]".to_string()));
  }

  #[test]
  fn compare_works() {
    assert_eq!(Ordering::Equal, compare(&make(&"[]".to_string()), &make(&"[]".to_string())));
    assert_eq!(Ordering::Equal, compare(&make(&"[1,2,3]".to_string()), &make(&"[1,2,3]".to_string())));
    assert_eq!(Ordering::Less, compare(&make(&"[1,1,3,1,1]".to_string()), &make(&"[1,1,5,1,1]".to_string())));
    assert_eq!(Ordering::Less, compare(&make(&"[[1],[2,3,4]]".to_string()), &make(&"[[1],4]".to_string())));
    assert_eq!(Ordering::Greater, compare(&make(&"[9]".to_string()), &make(&"[[8,7,6]]".to_string())));
    assert_eq!(Ordering::Less, compare(&make(&"[[4,4],4,4]".to_string()), &make(&"[[4,4],4,4,4]".to_string())));
    assert_eq!(Ordering::Greater, compare(&make(&"[7,7,7,7]".to_string()), &make(&"[7,7,7]".to_string())));
    assert_eq!(Ordering::Less, compare(&make(&"[]".to_string()), &make(&"[3]".to_string())));
    assert_eq!(Ordering::Greater, compare(&make(&"[[[]]]".to_string()), &make(&"[[]]".to_string())));
    assert_eq!(Ordering::Greater, compare(&make(&"[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string()), &make(&"[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string())));
    assert_eq!(Ordering::Less, compare(&make(&"[[5,4],4]".to_string()), &make(&"[[[5,4]],4]".to_string())));    
  }
}