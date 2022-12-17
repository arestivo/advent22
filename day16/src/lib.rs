use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Valve {
  pub label: String,
  pub tunnels: Vec<String>,
  pub rate: u32
}

pub fn line_to_valves(lines: &[String]) -> HashMap<String, Valve> {
  let mut valves = HashMap::new();
  let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((\w|,| )+)").unwrap();

  for line in lines {
    let cap = re.captures_iter(line).next().unwrap();
    let tunnels = cap[3].to_string().split(", ").map(|s| s.to_string()).collect();

    valves.insert(cap[1].to_string(), Valve { label: cap[1].to_string(), tunnels, rate: cap[2].to_string().parse().unwrap() });
  }

  valves
}

