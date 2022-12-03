pub fn priority(i: char) -> u32 {
  if i >= 'a' { i as u32 - 96 } else { i as u32 - 38 }
}