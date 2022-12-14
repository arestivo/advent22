use day10::lines_to_code;

fn main() {
  let lines = global::read_strings();
  let mut cpu = lines_to_code(&lines);

  for _ in 0..6 {
    for column in 0..40 {
      if (column - cpu.registers.x).abs() <= 1 { print!("#") }
      else { print!(" "); }
      cpu.cycle();
    }
    println!()
  }
}