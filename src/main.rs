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
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1]; // 0 is the name of the program
  let mut f=File::open(filename).expect("Error while opening the file.");
  let mut content = String::new();
  f.read_to_string(&mut content).expect("Error while reading the file.");
}

