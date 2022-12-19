use std::{cmp::max, collections::HashMap};

#[derive(Debug)]
pub struct Blueprint {
  pub ore: u64,
  pub clay: u64,
  pub obsidian: (u64, u64),
  pub geode: (u64, u64),
}

#[derive(Debug)]
pub struct Stock {
  pub ore: u64,
  pub clay: u64,
  pub obsidian: u64
}

impl Stock {
  pub fn new(ore: u64, clay: u64, obsidian: u64) -> Self { Stock { ore, clay, obsidian } }

  pub fn use_ore(&self, q: u64) -> Stock { Stock { ore: self.ore - q, clay: self.clay, obsidian: self.obsidian}}
  pub fn use_clay(&self, q: u64) -> Stock { Stock { ore: self.ore, clay: self.clay - q, obsidian: self.obsidian}}
  pub fn use_obsidian(&self, q: u64) -> Stock { Stock { ore: self.ore, clay: self.clay, obsidian: self.obsidian - q}}

  pub fn make_ore(&self) -> Stock { Stock { ore: self.ore + 1, clay: self.clay, obsidian: self.obsidian}}
  pub fn make_clay(&self) -> Stock { Stock { ore: self.ore, clay: self.clay + 1, obsidian: self.obsidian}}
  pub fn make_obsidian(&self) -> Stock { Stock { ore: self.ore, clay: self.clay, obsidian: self.obsidian + 1}}

  pub fn produce(&self, robots: &Stock) -> Stock { Stock { ore: self.ore + robots.ore, clay: self.clay + robots.clay, obsidian: self.obsidian + robots.obsidian}}

  pub fn hash(&self) -> String { format!("{},{},{}", self.ore, self.clay, self.obsidian )}
}

pub fn lines_to_blueprints(lines: &[String]) -> Vec<Blueprint> {
  let mut blueprints = vec![];

  for line in lines {
    let values = global::extract_numbers_from_string(line);
    blueprints.push( Blueprint {ore: values[1], clay: values[2], obsidian: (values[3], values[4]), geode: (values[5], values[6])});
  }

  blueprints
}

pub fn dps(mem: &mut HashMap<String, u64>, bp: &Blueprint, stock: &Stock, robots: &Stock, stop_at: &Stock, time: u64, geodes: u64) -> u64 {
  let hash = hash(stock, robots, time);

  if time == 0 { return geodes }

  if let Some(v) = mem.get(&hash) { return *v; }
  
  let mut best = 0;
  let next_stock = stock.produce(robots);

  if stock.ore >= bp.geode.0 && stock.obsidian >= bp.geode.1 { 
    best = max(best, dps(mem, bp, &next_stock.use_ore(bp.geode.0).use_obsidian(bp.geode.1), robots, stop_at, time - 1, geodes + time - 1)); 
  } else {
    if robots.obsidian < stop_at.obsidian && stock.ore >= bp.obsidian.0 && stock.clay >= bp.obsidian.1 { 
      best = max(best, dps(mem, bp, &next_stock.use_ore(bp.obsidian.0).use_clay(bp.obsidian.1), &robots.make_obsidian(), stop_at, time - 1, geodes)); 
    } 
    
    if robots.clay < stop_at.clay && stock.ore >= bp.clay { 
      best = max(best, dps(mem, bp, &next_stock.use_ore(bp.clay), &robots.make_clay(), stop_at, time - 1, geodes)); 
    } 
    
    if robots.ore < stop_at.ore &&stock.ore >= bp.ore { 
      best = max(best, dps(mem, bp, &next_stock.use_ore(bp.ore), &robots.make_ore(), stop_at, time - 1, geodes)); 
    } 
  
    best = max(best, dps(mem, bp, &next_stock, robots, stop_at, time - 1, geodes));  
  }

  mem.insert(hash, best);

  best
}

fn hash (stock: &Stock, robots: &Stock, time: u64) -> String {
  format!("{}:{}:{}", stock.hash(), robots.hash(), time)
}