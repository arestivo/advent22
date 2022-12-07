use std::{collections::HashMap, path::PathBuf};

use regex::Regex;


pub fn read_folder(lines: &Vec<String>) -> HashMap<String, u64> {
  let mut root = HashMap::new();
  let mut current = PathBuf::new();

  current.push("/");
  root.insert(current.as_path().display().to_string(), 0 as u64);

  let cd_folder = Regex::new(r"^\$ cd ([a-z]+)$").unwrap();
  let cd_back = Regex::new(r"^\$ cd \.\.$").unwrap();
  let file_entry = Regex::new(r"^(\d+) ([a-z.]+)$").unwrap();

  for line in lines {
    if cd_folder.is_match(&line) {
      let name = cd_folder.captures_iter(line).next().unwrap().get(1).unwrap().as_str();
      current.push(name);
      root.insert(current.as_path().display().to_string(), 0);
    }

    if cd_back.is_match(line) {
      current = current.parent().unwrap().to_path_buf();
    }

    if file_entry.is_match(line) {
      let result = file_entry.captures_iter(line).next().unwrap();

      let size = result.get(1).unwrap().as_str().parse::<u64>().unwrap();
      let _filename = result.get(2).unwrap().as_str();

      let mut pointer = current.clone();

      loop {
        let name = pointer.as_path().display().to_string();
        let previous = root.get(&name).unwrap();
  
        root.insert(name, previous + size);  
 
        if pointer.parent().is_none() { break };

        pointer = pointer.parent().unwrap().to_path_buf();
      }
    }
  }

  root
}