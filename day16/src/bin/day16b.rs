use std::collections::HashMap;
use day16::{line_to_valves, Valve};

fn main() {
  let lines = global::read_strings();
  let valves = line_to_valves(&lines);

  let mut mem = HashMap::new();

  let best = dfs(&valves, &mut mem, "AA", vec![], 26, 0);
  let best = best.0 + dfs(&valves, &mut mem, "AA", best.1, 26, 0).0;

  println!("{:?}", best);
}

fn hash(current: &str, opened: Vec<String>, remaining: u32) -> String {
  format!("{}:{}:{}", current, opened.join(","), remaining)
}

fn dfs(valves: &HashMap<String, Valve>, mem: &mut HashMap<String, (u32, Vec<String>)>, current: &str, opened: Vec<String>, remaining: u32, releasing: u32) -> (u32, Vec<String>) {
  let h = hash(current, opened.clone(), remaining);
  if mem.contains_key(&h) { return mem.get(&h).unwrap().clone() }

  let valve = valves.get(current).unwrap();
  let mut best = (0, vec![]);

  if remaining == 0 { 
    best = (0, opened); 
  } else if valves.iter().filter(|v| v.1.rate != 0).count() == opened.len() { 
    best = (remaining * releasing, opened);
  } else {
    if valve.rate != 0 && !opened.contains(&valve.label) {
      let mut opened = opened.clone();
      opened.push(valve.label.clone());
      opened.sort();
      let dfs = dfs(valves, mem, current, opened.clone(), remaining - 1, releasing + valve.rate);
      best = (dfs.0 + releasing, dfs.1);
    }

    for next in valve.tunnels.clone() {
      let dfs = dfs(valves, mem, &next, opened.clone(), remaining - 1, releasing);
      if releasing + dfs.0 > best.0 { best = (releasing + dfs.0, dfs.1) }
    }  
  }

  mem.insert(h, best.clone());

  best
}