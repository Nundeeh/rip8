use std::env;
use std::fs::File;

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
  let mut f=File::open(filename)?;
  let mut content = String::new();
  f.read_to_string(&mut content)?;
  for line in search(&config.query, &content) {
     println!("{}", line); 
}
