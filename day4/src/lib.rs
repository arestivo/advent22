use global::to_u32;

pub type Assignment = ((u32, u32), (u32, u32));

fn split(s: &str, d: char) -> (&str, &str) {
  s.split_once(d).unwrap()
}

fn parse_assignments(lines: Vec<String>) -> Vec<Assignment> {
  lines.iter()
    .map(|ap| split(ap, ','))
    .map(|p| (split(p.0, '-'), split(p.1, '-')))
    .map(|p| ((to_u32(p.0.0), to_u32(p.0.1)),(to_u32(p.1.0), to_u32(p.1.1))))
    .collect()
}

pub fn filter_assignements(lines: Vec<String>, filter: fn(&Assignment) -> bool) -> Vec<Assignment>{
  parse_assignments(lines)
    .into_iter()
    .filter(filter).to_owned().collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn split_works() {
    assert_eq!(("a", "b"), split("a,b", ','));
    assert_eq!(("1-2", "2-3"), split("1-2,2-3", ','));
    assert_eq!(("1", "2"), split("1-2", '-'));
  }

  #[test]
  fn parse_assignments_works() {
    assert_eq!(vec!(((1, 2), (2, 3)), ((4, 5),(6, 7))), parse_assignments(vec![String::from("1-2,2-3"), String::from("4-5,6-7")]));
  }

  #[test]
  fn filter_assignements_works() {
    assert_eq!(vec![((1, 2), (2, 3)), ((4, 5),(6, 7))], filter_assignements(vec!(String::from("1-2,2-3"), String::from("4-5,6-7")), |_v| true));
    assert_eq!(vec![] as Vec<Assignment>, filter_assignements(vec![String::from("1-2,2-3"), String::from("4-5,6-7")], |_v| false));
    assert_eq!(vec![((1, 2), (2, 3))], filter_assignements(vec!(String::from("1-2,2-3"), String::from("4-5,6-7")), |v| v.0.0 == 1));
  }
}