fn main() {
  let lines = global::read_strings();
  let trees = global::lines_to_array(lines);

  let visible = visibility(&trees);

  println!("{}", visible.iter().map(|r| r.iter().sum::<u32>()).sum::<u32>());
}

fn visibility(trees: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let height = trees.len();
  let width = trees[0].len();

  let mut visible = vec![vec![0; trees[0].len()]; trees.len()];

  (0..height).for_each(|r| {
    for c in 0..width {
      visible[r][c] = is_visible(trees, r, c);
    }
  });

  visible
}

fn is_visible(trees: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
  let mut count = 4;

  for r in 0..row { if trees[r][column] >= trees[row][column] { count -= 1; break; }}
  for r in row + 1..trees.len() { if trees[r][column] >= trees[row][column] { count -= 1; break; }}
  for c in 0..column { if trees[row][c] >= trees[row][column] { count -= 1; break; }}
  for c in column +1..trees[0].len() { if trees[row][c] >= trees[row][column] { count -= 1; break; }}

  if count == 0 { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_visible_works() {
    let lines = vec!["30373".to_string(), "25512".to_string(), "65332".to_string(), "33549".to_string(), "35390".to_string()];
    let trees = global::lines_to_array(lines);
    let visible = visibility(&trees);


    assert_eq!(visible, vec![vec![1, 1, 1, 1, 1], vec![1, 1, 1, 0, 1], vec![1, 1, 0, 1, 1], vec![1, 0, 1, 0, 1], vec![1, 1, 1, 1, 1]]);
  }
}