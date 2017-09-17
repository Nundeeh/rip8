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
    for x in 0..30 {
    self.fetch_opcode();
    self.run_opcode();
    }
  }

  fn fetch_opcode(&mut self) {
    self.opcode = (self.memory[self.pc as usize] as u16) << 8 
    | self.memory[(self.pc + 1) as usize] as u16;
  }

  fn run_opcode(&mut self) {
    match self.opcode & 0xF000 {
     0x1000 => {
      //1NNN: Jump to the address NNN  
      self.pc = self.opcode & 0x0FFF;
     }

     0x2000 =>  {
      //2NNN: call subroutine at NNN -> store pc on stack and jump to address NNN
      println!("opcode: {:X}, executed", self.opcode);
      self.stack[self.sp as usize] = self.pc;
      self.sp += 1;
      self.pc = self.opcode & 0x0FFF;
     }

     0x3000 => {
      //3XNN: skip next instruction if V[X] == NN  
      let x: u16 = self.register[(self.opcode & 0x0F00 >> 8) as usize] as u16;
      let y: u16 = (self.opcode & 0x00FF) as u16;

      if x == y {
        self.pc += 4; 
      }
      else {
        self.pc += 2;
      }
     }

     0x4000 => {
      //4XNN: skip the next instruction if V[X] != NN 
      let x: u16 = self.register[(self.opcode & 0x0F00 >> 8) as usize] as u16;
      let y: u16 = (self.opcode & 0x00FF) as u16;

      if x != y {
        self.pc += 4;
      }
      else {
        self.pc += 2; 
      }
     }

     0x5000 => {
      //5XY0: skip thenext instruction if V[X] == V[Y] 
      let x: u16 = self.register[(self.opcode & 0x0F00 >> 8) as usize] as u16;
      let y: u16 = self.register[(self.opcode & 0x00F0 >> 4) as usize] as u16;

      if x == y {
        self.pc += 4; 
      }
      else {
        self.pc += 2;
      }
     }

     0x6000 => {
      println!("opcode: {:X}, executed", self.opcode);
      self.register[(self.opcode & 0x0F00 >> 8) as usize] = (self.opcode & 0x00FF) as u8;
      self.pc += 2; 
     }

     0xA000 => {
      println!("opcode: {:X}, executed", self.opcode);
      self.index = self.opcode & 0x0FFF;
      self.pc += 2;
     }

     0xD000 => {
      //Waiting with the drawing stuff until later, just increase pc for now
      println!("opcode: {:X}, not implemented yet", self.opcode);
      self.pc += 2;
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

