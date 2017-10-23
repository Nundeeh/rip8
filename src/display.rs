extern crate sdl2;

pub struct Display {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub sdl_context: sdl2::Sdl,
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Rip-8",800,800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        Display {
            canvas,
            sdl_context,
        }
    }
    
    pub fn render(&mut self, display: [bool; 30*64]) {
       self.canvas.clear();
       self.canvas.present();
    }

}
