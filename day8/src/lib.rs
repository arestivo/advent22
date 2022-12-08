pub fn lines_to_array(lines: Vec<String>) -> Vec<Vec<Option<u32>>> {
  let mut trees = vec![];

  for line in lines {
    let mut row = vec![];
    for c in line.chars() {
      row.push(c.to_digit(10));
    }
    trees.push(row);
  }

  trees
}