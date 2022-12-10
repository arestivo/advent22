use day10::lines_to_code;

fn main() {
  let lines = global::read_strings();
  let mut cpu = lines_to_code(&lines);

  let mut cycle = 1;
  let mut total = 0;

  while !cpu.finished() {
    if (cycle - 20) % 40 == 0 { total += cycle * cpu.registers.x; }
    cpu.cycle();
    cycle += 1;
  }

  println!("{}", total)
}