use std::{cmp::max, collections::HashMap};

type Stock = [u64; 3];
type Robots = [u64; 3];
type Instructions = [u64; 3];
type BluePrint = [Stock; 4];

pub fn lines_to_blueprints(lines: &[String]) -> Vec<BluePrint> {
  let mut blueprints = vec![];

  for line in lines {
    let values = global::extract_numbers_from_string(line);
    blueprints.push([
        [values[1], 0, 0], 
        [values[2], 0, 0], 
        [values[3], values[4], 0], 
        [values[5], 0, values[6]] 
    ]);
  }

  blueprints
}

fn produce(stock: Stock, robots: Robots, time: u64) -> Stock {
  let mut next = stock;
  for i in 0..3 { next[i] += robots[i] * time; }
  next
}

fn consume(stock: Stock, instructions: Instructions) -> Stock {
  let mut next = stock;
  for i in 0..3 { next[i] -= instructions[i]; }
  next
}

fn build(robots: Robots, orb: usize) -> Stock {
  let mut next = robots;
  next[orb] += 1;
  next
}

fn should_build(orb: usize, stop_at: Robots, robots: Robots) -> bool {
  stop_at[orb] > robots[orb]
}

fn can_build(instructions: Instructions, stock: Stock) -> bool {
  instructions.iter().zip(&stock).all(|(i, s)| s >= i)
}

fn time_to_build(instructions: Instructions, stock: Stock, robots: Robots) -> u64 {
  instructions.iter()
    .zip(&stock).map(|(i, s)| if s >= i {0} else {i - s})
    .zip(&robots).map(|(n, r)| if n > 0 && *r == 0 { u64::MAX } else {(n as f64 / *r as f64).ceil() as u64})
    .max().unwrap()
}

fn potential(time: u64) -> u64 {
  time * (time - 1) / 2
}

pub struct DfsEnv { pub best: u64, pub bp: BluePrint, pub mem: HashMap<(Stock, Robots, u64), u64>, pub stop_at: Robots }

pub fn dps(stock: Stock, robots: Robots, time: u64, geodes: u64, env: &mut DfsEnv) -> u64 {
  let hash = (stock, robots, time);

  if time == 0 { return geodes }
  if let Some(v) = env.mem.get(&hash) { return *v; }
  if geodes + potential(time) <= env.best { return 0 }
  
  let mut best = 0;

  if can_build(env.bp[3], stock) {
    let mut next_stock = produce(stock, robots, 1);
    next_stock = consume(next_stock, env.bp[3]);
    best = max(best, dps(next_stock, robots, time - 1, geodes + time - 1, env)); 
  } else {
    for orb in 0..3 {
      if should_build(orb, env.stop_at, robots) { 
        let ttb = time_to_build(env.bp[orb], stock, robots);
        if ttb < time {
          let stock_after_producing = produce(stock, robots, ttb + 1);
          let stock_after_consuming = consume(stock_after_producing, env.bp[orb]);
          let next_robots = build(robots, orb);
          best = max(best, dps(stock_after_consuming, next_robots, time - ttb - 1, geodes, env));     
        }
      } 
    }
      
    let next_stock = produce(stock, robots, 1);
    best = max(best, dps(next_stock, robots, time - 1, geodes, env));  
  }

  env.best = max(env.best, best);
  env.mem.insert(hash, best);
  best
}