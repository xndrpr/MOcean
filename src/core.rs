use piston::{
    EventSettings, Events, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings,
};
use piston_window::rectangle;
use piston_window::*;

pub struct Core {
    window: PistonWindow,
}

impl Core {
    pub fn new() -> Self {
        const WIDTH: u32 = 850;
        const HEIGHT: u32 = 450;

        let window: PistonWindow = WindowSettings::new("MOcean", [WIDTH, HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Self { window: window }
    }

    fn render(&mut self, args: &RenderArgs) {
        let BLACK = [0.2, 0.2, 0.2, 1.0];
        let WHITE = [1.0, 1.0, 1.0, 1.0];

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let mut glyphs = self.window.load_font(assets.join("font.ttf")).unwrap();

        self.window.set_lazy(true);
        while let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g, device| {
                let transform = c.transform.trans(10.0, 100.0);

                clear([1.0, 1.0, 1.0, 1.0], g);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                    .draw("Hello world!", &mut glyphs, &c.draw_state, transform, g)
                    .unwrap();

                glyphs.factory.encoder.flush(device);
            });

            self.window.draw_2d(&e, |c, g, _| {
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [0.0, 0.0, 100.0, 100.0],
                    c.transform,
                    g,
                );
            });
        }
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
