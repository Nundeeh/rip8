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
  sp: u16
}

impl Chip8 {
  fn new(op_code: Vec<u8>) -> Chip8 {
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
   }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = &args[1]; // 0 is the name of the program

  let mut f = File::open(&path).expect("Error while opening file.");
  let mut op_code = Vec::new();

  f.read_to_end(&mut op_code).expect("Error while reading file.");

  let chip8 = Chip8::new(op_code);
  for byte in chip8.memory.iter().enumerate() {
    println!("{:?}",byte);
  }
}

fn decode (op_code: Vec<u8>) {
    
}
