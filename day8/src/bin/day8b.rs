use std::cmp::max;

fn main() {
  let lines = global::read_strings();
  let trees = global::lines_to_array(lines);

  let mut best = 0;

  for r in 0..trees.len() {
    for c in 0..trees[0].len() {
      best = max(best, scenic_score(&trees, r, c));
    }
  }

  println!("{}", best);
}

fn scenic_score(trees: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
  let mut score = 1;

  let mut count = 0;
  for r in (0..row).rev() { count += 1; if trees[r][column] >= trees[row][column] { break } }
  score *= count;

  let mut count = 0;
  for r in row + 1..trees.len() { count += 1; if trees[r][column] >= trees[row][column] { break; } }
  score *= count;

  let mut count = 0;
  for c in (0..column).rev() { count += 1; if trees[row][c] >= trees[row][column] { break; } }
  score *= count;

  let mut count = 0;
  for c in column +1..trees[0].len() { count += 1; if trees[row][c] >= trees[row][column] { break; } }
  score *= count;

  score
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scenic_score_works() {
    let lines = vec!["30373".to_string(), "25512".to_string(), "65332".to_string(), "33549".to_string(), "35390".to_string()];
    let trees = global::lines_to_array(lines);

    let mut scores = vec![vec![0; trees[0].len()]; trees.len()];

    for r in 0..trees.len() {
      for c in 0..trees[0].len() {
        scores[r][c] = scenic_score(&trees, r, c);
      }
    }

    assert_eq!(scores, vec![vec![0, 0, 0, 0, 0], vec![0, 1, 4, 1, 0], vec![0, 6, 1, 2, 0], vec![0, 1, 8, 3, 0], vec![0, 0, 0, 0, 0]]);
  }
}