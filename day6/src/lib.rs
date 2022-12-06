use std::collections::HashSet;

pub fn find_marker(packet: String, size: usize) -> usize {
  let mut previous = vec![' '; size];

  for (i, c) in packet.chars().enumerate() {
    previous[i % size] = c;
    if previous[size - 1] != ' ' && HashSet::<char>::from_iter(previous.clone()).len() == size { 
      return i + 1; 
    }
  }

  unreachable!()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_marker_works() {
    assert_eq!(7, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4));
    assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4));
    assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4));
    assert_eq!(10, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4));
    assert_eq!(11, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4));

    assert_eq!(19, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14));
    assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14));
    assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14));
    assert_eq!(29, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14));
    assert_eq!(26, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14));
  }
}