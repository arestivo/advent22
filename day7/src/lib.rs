use std::{collections::HashMap, path::PathBuf};

use regex::Regex;


pub fn read_folders(lines: &Vec<String>) -> HashMap<String, u64> {
  let mut folders = HashMap::new();
  let mut current = "".to_string();

  folders.insert("".to_string(), 0 as u64);

  let cd_folder = Regex::new(r"^\$ cd ([a-z]+)$").unwrap();
  let cd_back = Regex::new(r"^\$ cd \.\.$").unwrap();
  let file_entry = Regex::new(r"^(\d+) [a-z.]+$").unwrap();

  for line in lines {
    if cd_folder.is_match(&line) {
      let name = cd_folder.captures_iter(line).next().unwrap().get(1).unwrap().as_str();

      current = format!("{}/{}", &current, name);

      folders.insert(current.clone(), 0);
    }

    if cd_back.is_match(line) {
      current = PathBuf::from(current).parent().unwrap().display().to_string();
    }

    if file_entry.is_match(line) {
      let result = file_entry.captures_iter(line).next().unwrap();
      let size = result.get(1).unwrap().as_str().parse::<u64>().unwrap();
      folders.iter_mut().for_each(|p| if current.starts_with(p.0) { *p.1 = *p.1 + size });
    }
  }

  folders
}