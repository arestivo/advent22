#[derive(Debug)]
pub enum OpCode {
  Noop,
  AddX { value: i64 }
}

#[derive(Debug)]
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