use std::{cmp::max, collections::HashMap};
use day16::{line_to_valves, Valve};

fn main() {
  let lines = global::read_strings();
  let valves = line_to_valves(&lines);

  let mut mem = HashMap::new();

  let best = dfs(&valves, &mut mem, "AA", vec![], 30, 0);

  println!("{}", best);
}

fn hash(current: &str, opened: Vec<String>, remaining: u32) -> String {
  format!("{}:{}:{}", current, opened.join(","), remaining)
}

fn dfs(valves: &HashMap<String, Valve>, mem: &mut HashMap<String, u32>, current: &str, opened: Vec<String>, remaining: u32, releasing: u32) -> u32 {
  let h = hash(current, opened.clone(), remaining);
  if mem.contains_key(&h) { return *mem.get(&h).unwrap(); }

  let valve = valves.get(current).unwrap();
  let mut best = 0;

  if remaining == 0 { 
    best = 0; 
  } else if valves.iter().filter(|v| v.1.rate != 0).count() == opened.len() { 
    best = remaining * releasing 
  } else {
    if valve.rate != 0 && !opened.contains(&valve.label) {
      let mut opened = opened.clone();
      opened.push(valve.label.clone());
      opened.sort();

      best = releasing + dfs(valves, mem, current, opened.clone(), remaining - 1, releasing + valve.rate);
    }

    for next in valve.tunnels.clone() {
      best = max(best, releasing + dfs(valves, mem, &next, opened.clone(), remaining - 1, releasing));  
    }  
  }

  mem.insert(h, best);

  best
}