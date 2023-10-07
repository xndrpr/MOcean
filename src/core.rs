use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    EventSettings, Events, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings,
};

pub struct Core {
    window: Window,
    gl: GlGraphics,
}

impl Core {
    pub fn new() -> Self {
        const WIDTH: u32 = 850;
        const HEIGHT: u32 = 450;

        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("U3D Engine", [WIDTH, HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(false)
            .build()
            .unwrap();

        Self {
            gl: GlGraphics::new(opengl),
            window: window,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let BLACK = [0.2, 0.2, 0.2, 1.0];
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
        });
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {}
}
