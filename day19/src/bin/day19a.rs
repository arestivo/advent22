use std::collections::HashMap;

use day19::{lines_to_blueprints, DfsEnv};

fn main() {
  let lines = global::read_strings();
  let bps = lines_to_blueprints(&lines);

  let mut sum = 0;
  let mut id = 1;

  for bp in bps {
    let stop_at = [ 
      (0..=3).map(|o| bp[o][0]).max().unwrap(),
      (0..=3).map(|o| bp[o][1]).max().unwrap(),
      (0..=3).map(|o| bp[o][2]).max().unwrap()
    ];

    let mut env = DfsEnv { best: 0, bp, mem: HashMap::new(), stop_at };

    let geodes = day19::dps(
      [0, 0, 0], 
      [1, 0, 0], 
      24, 0, &mut env);
    sum += id * geodes;
    id += 1;
  }

  println!("{:?}", sum);
}