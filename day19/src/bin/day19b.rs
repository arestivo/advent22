use std::{collections::HashMap};

use day19::{lines_to_blueprints, dps, DfsEnv};

fn main() {
  let lines = global::read_strings();
  let bps = lines_to_blueprints(&lines);

  let mut mul = 1;

  for bp in &bps[0..3] {
    let mut env = DfsEnv { best: 0, bp: *bp, mem: HashMap::new() };

    let stop_at = [ 
      (0..=3).map(|o| bp[o][0]).max().unwrap(),
      (0..=3).map(|o| bp[o][1]).max().unwrap(),
      (0..=3).map(|o| bp[o][2]).max().unwrap()
    ];

    let geodes = dps(
      [0, 0, 0], 
      [1, 0, 0], 
      stop_at,
      32, 0, &mut env);
    mul *= geodes;
  }

  println!("{:?}", mul);
}