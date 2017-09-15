use std::env;
use std::fs::File;
use std::io::prelude::*;

 struct Chip8 {
  memory: [u8; 4096], 
  register: [u8; 16],
  index: u16,
  pc: u16,
  display: [u8; 30*64],
  stack: [u16; 16],
  sp: u16,
  opcode: u16
}

impl Chip8 {
  fn new(op_code: Vec<u8>) ->  Chip8 {
    let mut memory: [u8; 4096] = [0; 4096];

    for (i, byte) in op_code.iter().enumerate() {
      memory[0x200 + i] = byte.clone();
    }

     Chip8 {
      memory,
      register: [0; 16],
      index: 0, 
      pc: 0x200,   
      display: [0; 30*64],
      stack: [0; 16],
      sp: 0,
      opcode: 0
   }
  }

  fn run_cycle(&mut self) {
    self.fetch_opcode();
    self.run_opcode();
  }

  fn fetch_opcode(&mut self) {
    self.opcode = (self.memory[self.pc as usize] as u16) << 8 
    | self.memory[(self.pc + 1) as usize] as u16;
  }

  fn run_opcode(&mut self) {
    match self.opcode & 0xF000 {
     0x2000 =>  {
      //2NNN: Jump to NNN 
      self.stack[self.sp as usize] = self.pc;
      self.sp += 1;
      self.pc = self.opcode & 0x0FFF;
     }

     _ => println!("opcode: {:X},not implemented yet", self.opcode)

    }
  }
}

fn read_rom(path: String) -> Vec<u8> {
  let mut f = File::open(&path).expect("Error while opening file.");
  let mut rom = Vec::new();
  f.read_to_end(&mut rom).expect("Error while reading file.");
  rom
}

fn main() {
  let path: String = env::args().nth(1).unwrap(); 
  let rom = read_rom(path);

  let mut chip8 = Chip8::new(rom);
  chip8.run_cycle();
}

