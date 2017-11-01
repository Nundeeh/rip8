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

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'main
                }

                _ => {}
            } 
        }

        let mut keys = HashSet::new();
        keys = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        println!("{:?}", keys);

        chip8.run_cycle();
        if chip8.draw_flag {
            display.render(chip8.display);
        }
    }
}
