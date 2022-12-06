fn main() {
  let line = global::read_single_line();

  println!("{}", day6::find_marker(line, 14));
}