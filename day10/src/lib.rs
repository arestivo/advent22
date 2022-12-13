#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
  Noop,
  AddX { value: i64 }
}

#[derive(Debug, PartialEq, Eq)]
pub struct  Instruction {
  cycles: usize,
  opcode: OpCode
}

#[derive(Debug)]
pub struct Registers {
  pub x: i64
}

#[derive(Debug)]
pub struct CPU {
  pc: usize,
  pub registers: Registers,
  code: Vec<Instruction>,
  cycles: usize
}

pub fn lines_to_code(lines: &Vec<String>) -> CPU {
  let mut cpu = CPU { pc: 0, registers: Registers { x: 1 }, code: vec![], cycles: 0 };

  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();
    match parts[0] {
      "noop" => { cpu.code.push(Instruction::new(1, OpCode::Noop)) }
      "addx" => { cpu.code.push(Instruction::new(2, OpCode::AddX { value: parts[1].parse().unwrap() })) }
      &_ => unreachable!()
    }
  }

  cpu.cycles = cpu.code[0].cycles;

  cpu
}

impl Instruction {
  fn new(cycles: usize, opcode: OpCode) -> Instruction { Instruction { cycles, opcode } }
}

impl CPU {
  pub fn finished(&self) -> bool {
    self.pc >= self.code.len()
  }

  pub fn cycle(&mut self) {
    if self.finished() { panic!() }
    self.execute_current();
    if self.cycles == 0 { self.load_next() }
  }

  fn load_next(&mut self) {
    self.pc += 1;
    if !self.finished() { self.cycles = self.code[self.pc].cycles }
  }

  fn execute_current(&mut self) {
    if self.finished() { panic!() }
    if self.cycles == 1 {
      match self.code[self.pc].opcode {
        OpCode::Noop => {},
        OpCode::AddX{ value } => { self.registers.x += value }
      }
    }
    self.cycles -= 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lines_to_code_works() {
    let cpu = lines_to_code(&vec!["addx 10".to_string(), "addx -1".to_string(), "noop".to_string()]);

    assert_eq!(0, cpu.pc);
    assert_eq!(vec![
      Instruction{cycles: 2, opcode: OpCode::AddX{value: 10}},
      Instruction{cycles: 2, opcode: OpCode::AddX{value: -1}},
      Instruction{cycles: 1, opcode: OpCode::Noop}
    ], cpu.code);
    assert_eq!(1, cpu.registers.x);
    assert_eq!(2, cpu.cycles);
  }

  #[test]
  fn cycle_works() {
    let mut cpu = lines_to_code(&vec!["addx 10".to_string(), "addx -1".to_string(), "noop".to_string()]);

    cpu.cycle();
    assert_eq!(1, cpu.registers.x);
    assert_eq!(1, cpu.cycles);
    assert!(!cpu.finished());

    cpu.cycle();
    assert_eq!(11, cpu.registers.x);
    assert_eq!(2, cpu.cycles);
    assert!(!cpu.finished());

    cpu.cycle();
    assert_eq!(11, cpu.registers.x);
    assert_eq!(1, cpu.cycles);
    assert!(!cpu.finished());

    cpu.cycle();
    assert_eq!(10, cpu.registers.x);
    assert_eq!(1, cpu.cycles);
    assert!(!cpu.finished());

    cpu.cycle();
    assert_eq!(10, cpu.registers.x);
    assert_eq!(0, cpu.cycles);
    assert!(cpu.finished());

  }

}