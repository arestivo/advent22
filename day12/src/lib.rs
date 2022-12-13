use priority_queue::PriorityQueue;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
  pub r: usize,
  pub c: usize
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State {
  pub pos: Position,
  pub moves: u32
}

pub fn lines_to_heights(lines: Vec<String>) -> (Vec<Vec<u32>>, Position, Position) {
  let mut heights = vec![];
  let mut start = Position {r: 0, c: 0};
  let mut end = Position {r: 0, c: 0};

  for r in 0..lines.len() {
    let mut row = vec![];
    for c in 0..lines[r].len() {
      match lines[r].chars().nth(c).unwrap() {
        'S' => { row.push(0); start = Position { r, c } }
        'E' => { row.push(25);  end = Position { r, c } }
        l => { row.push(l as u32 - 'a' as u32) }
      }
    }
    heights.push(row);
  }

  (heights, start, end)
}

pub fn fastest_path(heights: &Vec<Vec<u32>>, start: &Position, end: &Position) -> u32 {
  let mut pq = PriorityQueue::new();
  let mut visited = vec![vec![u32::MAX; heights[0].len()]; heights.len()];

  visited[start.r][start.c] = 0;
  pq.push(State { pos: start.clone(), moves: 0 }, 0);

  while !pq.is_empty() {
    let (current, _) = pq.pop().unwrap();
    let height = heights[current.pos.r][current.pos.c];

    if &current.pos == end { return current.moves }

    for (dr, dc) in [(0, 1), (0, -1 as i32), (1, 0), (-1 as i32, 0)] {
      if current.pos.r as i32 + dr < 0 || current.pos.c as i32 + dc < 0 { continue }

      let next = State { pos: Position { r: (current.pos.r as i32 + dr) as usize, c: (current.pos.c as i32 + dc) as usize }, moves: current.moves + 1 };

      if next.pos.r >= heights.len() || next.pos.c >= heights[0].len() { continue }
      if heights[next.pos.r][next.pos.c] > height + 1 { continue }
      if next.moves > visited[next.pos.r][next.pos.c] { continue }

      visited[next.pos.r][next.pos.c] = next.moves;
      pq.push(next.clone(), u32::MAX - next.moves);
    }
  }

  u32::MAX 
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lines_to_heights_works() {
    assert_eq!((
      vec![vec![0, 1, 2],vec![3, 4, 5],vec![6, 7, 25]], 
      Position {r: 0, c: 0}, Position {r: 2, c: 2}), 
      lines_to_heights(vec!["Sbc".to_string(),"def".to_string(),"ghE".to_string()])
    );
  }

  #[test]
  fn fastest_path_works() {
    let (heights, start, end) = lines_to_heights(
      vec!["Sbcde".to_string(),"jihgf".to_string(),"klmno".to_string(),"tsrqp".to_string(),"uvwxy".to_string(),"Ezzzz".to_string()]);
    assert_eq!(29, fastest_path(&heights, &start, &end));

    let (heights, start, end) = lines_to_heights(
      vec!["Sbcde".to_string(),"Eihgf".to_string(),"yjklm".to_string(),"zqpon".to_string(),"yrstu".to_string(),"zyxwv".to_string()]);
    assert_eq!(29, fastest_path(&heights, &start, &end));
  }

}