use std::{collections::HashMap};

use day19::{lines_to_blueprints, Stock, dps};

fn main() {
  let lines = global::read_strings();
  let bps = lines_to_blueprints(&lines);

  let mut mul = 1;

  for bp in &bps[0..3] {
    let mut mem: HashMap<String, u64> = HashMap::new();

    let stop_at = Stock { 
      ore: *vec![bp.ore, bp.clay, bp.obsidian.0, bp.geode.0].iter().max().unwrap(),
      clay: bp.obsidian.1,
      obsidian: bp.geode.1
    };

    let geodes = dps(
      &mut mem, bp, 
      &Stock::new(0, 0, 0), 
      &Stock::new(1, 0, 0), 
      &stop_at,
      32, 0);
    mul *= geodes;
  }

  println!("{:?}", mul);
}