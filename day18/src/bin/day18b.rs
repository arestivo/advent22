use std::collections::HashSet;

use day18::Position;

fn main() {
  let lines = global::read_strings();
  let cubes = day18::lines_to_cubes(&lines);
  let exists: HashSet<&Position> = HashSet::from_iter(cubes.iter());
  let mut found: HashSet<Position> = HashSet::new();

  let l1 = Position { 
    x: cubes.iter().map(|c| c.x).min().unwrap() - 1, 
    y: cubes.iter().map(|c| c.y).min().unwrap() - 1, 
    z: cubes.iter().map(|c| c.z).min().unwrap() - 1 
  };

  let l2 = Position { 
    x: cubes.iter().map(|c| c.x).max().unwrap() + 1, 
    y: cubes.iter().map(|c| c.y).max().unwrap() + 1, 
    z: cubes.iter().map(|c| c.z).max().unwrap() + 1 
  };

  for x in l1.x..=l2.x {
    for y in l1.y..=l2.y {
      for z in l1.z..=l2.z {
        let air = Position { x, y, z };
        if !exists.contains(&air) { found.insert(air); }
      }
    }    
  }

  let mut to_visit = vec![l1.clone()];

  while !to_visit.is_empty() {
    let c = to_visit.pop().unwrap();
    let sides = c.neighbours();
    for s in sides {
      if !inside(&l1, &l2, &s) { continue; }
      else if !exists.contains(&s) && found.contains(&s) { 
        to_visit.push(s.clone()); 
        found.remove(&s.clone()); 
      }
    }
  }

  let total = cubes.iter()
    .map(|c| c.neighbours().iter().filter(|s| !exists.contains(s)).count())
    .sum::<usize>();

  let air = found.iter()
    .map(|c| c.neighbours().iter().filter(|s| !found.contains(s)).count())
    .sum::<usize>();  

  println!("{}", total - air);
}

fn inside(l1: &Position, l2: &Position, s: &Position) -> bool {
  l1.x <= s.x && l1.y <= s.y && l1.z <= s.z &&
  l2.x >= s.x && l2.y >= s.y && l2.z >= s.z
}
