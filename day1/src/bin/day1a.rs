use global;

fn main() {
  let max = calculate_max(&global::read_strings());

  println!("{}", max);
}


fn calculate_max(lines: &Vec<String>) -> i32 {
  lines
    .split(|l| l == "")
    .map(|e| e.iter().map(|s| s.parse::<i32>().unwrap()))
    .map(|v| v.sum::<i32>())
    .max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_max_works() {
    let lines: Vec<String> = [
      "1000", "2000", "3000", "", 
      "4000", "", 
      "5000", "6000", "", 
      "7000", "8000", "9000", "", 
      "10000"]
      .iter().map(|s| String::from(*s)).collect();
    assert_eq!(24000, calculate_max(&lines));
  }
}