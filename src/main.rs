extern crate sdl2;

use chip8::Chip8;
use display::Display;

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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
            for (keycode, new_key) in &KEY_CODES_DOWN {
                match event {
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => {
                        if k == *keycode {
                            key = *new_key;
                            break;
                        }
                    }
                    _ => (),
                }
            }
            for (keycode, new_key) in &KEY_CODES_UP {
                match event {
                    Event::KeyUp {
                        keycode: Some(k), ..
                    } => {
                        if k == *keycode {
                            key = *new_key;
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }
        chip8.run_cycle(old_key, &mut event_pump);
        if chip8.draw_flag {
            display.render(chip8.display);
        }
        if key != old_key {
            old_key = key;
        }
    }
}

const KEY_CODES_DOWN: [(Keycode, u8); 16] = [
    (Keycode::Num1, 0x00),
    (Keycode::Num2, 0x01),
    (Keycode::Num3, 0x02),
    (Keycode::Num4, 0x03),
    (Keycode::Q, 0x04),
    (Keycode::W, 0x05),
    (Keycode::E, 0x06),
    (Keycode::R, 0x07),
    (Keycode::A, 0x08),
    (Keycode::S, 0x09),
    (Keycode::D, 0x0A),
    (Keycode::F, 0x0B),
    (Keycode::Y, 0x0C),
    (Keycode::X, 0x0D),
    (Keycode::C, 0x0E),
    (Keycode::V, 0x0F),
];

const KEY_CODES_UP: [(Keycode, u8); 16] = [
    (Keycode::Num1, 0x10),
    (Keycode::Num2, 0x10),
    (Keycode::Num3, 0x10),
    (Keycode::Num4, 0x10),
    (Keycode::Q, 0x10),
    (Keycode::W, 0x10),
    (Keycode::E, 0x10),
    (Keycode::R, 0x10),
    (Keycode::A, 0x10),
    (Keycode::S, 0x10),
    (Keycode::D, 0x10),
    (Keycode::F, 0x10),
    (Keycode::Y, 0x10),
    (Keycode::X, 0x10),
    (Keycode::C, 0x10),
    (Keycode::V, 0x10),
];
