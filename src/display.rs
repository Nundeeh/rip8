extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Display {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub sdl_context: sdl2::Sdl,
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Rip-8",640,320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas,
            sdl_context,
        }
    }
    
    pub fn render(&mut self, display: [bool; 64*32]) {
        self.canvas.set_draw_color(Color::RGB(0,0,0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255,255,255));
        for y in 0..32 {
            for x in 0..64 {
                if display[y*64+x] {
                    self.canvas.fill_rect(Rect::new((x * 10) as i32,(y * 10) as i32,10,10)).expect("Error while drawing rectangle!");
                }
            } 
        } 
        self.canvas.present();
    }

}
