use std::{thread, time::Duration};

use piston::WindowSettings;
use piston_window::*;

use crate::widgets::input::Input as WInput;

pub struct Core {
    window: PistonWindow,
}

impl Core {
    const WIDTH: u32 = 850;
    const HEIGHT: u32 = 450;

    pub fn new() -> Self {
        let window: PistonWindow = WindowSettings::new("MOcean", [Self::WIDTH, Self::HEIGHT])
            .exit_on_esc(false)
            .resizable(false)
            .build()
            .unwrap();

        Self { window: window }
    }

    pub async fn run(&mut self) {
        let BLACK = [0.2, 0.2, 0.2, 1.0];
        let WHITE = [1.0, 1.0, 1.0, 1.0];

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let mut glyphs = self.window.load_font(assets.join("font.ttf")).unwrap();

        let mut input = WInput::new().await;
        while let Some(e) = self.window.next() {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                input.press(key).await;
            }
            if let Some(Button::Keyboard(key)) = e.release_args() {
                input.release(key).await;
            }

            self.window.draw_2d(&e, |c, g, device| {
                let transform = c.transform.trans(
                    Self::WIDTH as f64 / 2.0 - (input.text.len() * 10) as f64,
                    Self::HEIGHT as f64 / 2.0,
                );

                clear(BLACK, g);
                text::Text::new_color(WHITE, 32)
                    .draw(&input.text, &mut glyphs, &c.draw_state, transform, g)
                    .unwrap();

                glyphs.factory.encoder.flush(device);
            });
        }
    }
}
