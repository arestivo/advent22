use std::{collections::HashMap};

use day17::{Chamber, Rock};

fn main() {
  let line = global::read_single_line();
  let shapes = day17::create_rock_shapes();

  let mut chamber = Chamber::default();
  let mut rock = Rock::first(&chamber, &shapes);

  let mut gush = 0;
  let mut cycle = vec![];
  let mut cycles: HashMap<Vec<usize>, (usize, u64)> = HashMap::new();

  let mut cycle_height = 0;
  let mut cycle_count = 0;

  let mut r = 0;

  while r < 1000000000000_u64 {
    if !cycle.is_empty() && gush < *cycle.last().unwrap() {
      if cycles.contains_key(&cycle) && cycle_height == 0 { 
        let cycle_size = r - cycles.get(&cycle).unwrap().1 as u64;
        let start_cycle_height = cycles.get(&cycle).unwrap().0; 

        cycle_height = chamber.top - start_cycle_height;
        cycle_count = (1000000000000_u64 - r) / cycle_size;
        r += (cycle_count - 1) * cycle_size;
      }

      cycles.insert(cycle.clone(), (chamber.top, r));
      cycle.clear();
    }
    cycle.push(gush);

    loop {
      let c = line.chars().nth(gush).unwrap();
      gush = (gush + 1) % line.len();

      match c {
        '<' => { rock = rock.left(&chamber) }
        '>' => { rock = rock.right(&chamber) }
        _ => unreachable!()
      }

      rock = rock.down(&chamber);
      if rock.landed { break; }
    }

    chamber.add(rock.clone());
    rock = rock.next(&chamber, &shapes).clone();    
    r+=1;
  }

  println!("{}", chamber.top as u64 + (cycle_count - 1) * cycle_height as u64);
}