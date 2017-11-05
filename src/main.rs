extern crate sdl2;

use display::Display;
use chip8::Chip8;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod chip8;
mod display;

fn read_rom(path: String) -> Vec<u8> {
    let mut f = File::open(&path).expect("Error while opening file.");
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).expect("Error while reading file.");
    rom
}

fn main() {
    let path: String = env::args().nth(1).unwrap(); 
    let rom = read_rom(path);
    
    let mut chip8 = Chip8::new(rom, false);
    let mut display = Display::new();

    let mut event_pump = display.sdl_context.event_pump().unwrap();
    
    //variables for pressed keys
    let mut key = 0x10;
    let mut old_key = 0x10;
    'main: loop {
        key = 0x10;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'main
                }
                Event::KeyDown {keycode: Some(Keycode::Num1), ..} => {key = 0x00;},
                Event::KeyDown {keycode: Some(Keycode::Num2), ..} => {key = 0x01;},
                Event::KeyDown {keycode: Some(Keycode::Num3), ..} => {key = 0x02;},
                Event::KeyDown {keycode: Some(Keycode::Num4), ..} => {key = 0x03;},
                Event::KeyDown {keycode: Some(Keycode::Q), ..} => {key = 0x04;},
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {key = 0x05;},
                Event::KeyDown {keycode: Some(Keycode::E), ..} => {key = 0x06;},
                Event::KeyDown {keycode: Some(Keycode::R), ..} => {key = 0x07;},
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {key = 0x08;},
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {key = 0x09;},
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {key = 0x0A;},
                Event::KeyDown {keycode: Some(Keycode::F), ..} => {key = 0x0B;},
                Event::KeyDown {keycode: Some(Keycode::Y), ..} => {key = 0x0C;},
                Event::KeyDown {keycode: Some(Keycode::X), ..} => {key = 0x0D;},
                Event::KeyDown {keycode: Some(Keycode::C), ..} => {key = 0x0E;},
                Event::KeyDown {keycode: Some(Keycode::V), ..} => {key = 0x0F;},
                _ => {}
            }
        }
        chip8.run_cycle(key, &mut event_pump);
        if chip8.draw_flag {
            display.render(chip8.display);
        }
        old_key = key;
    }
}
