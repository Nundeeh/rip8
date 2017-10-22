use chip8::Chip8;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod chip8;

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
    chip8.run();
}
