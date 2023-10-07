use std::{thread, time::Duration};

use piston::WindowSettings;
use piston_window::*;

use crate::widgets::input::Input as WInput;

pub struct Core {
    window: PistonWindow,
    width: f64,
    height: f64,
}

impl Core {
    pub fn new() -> Self {
        let width = 850.0;
        let height = 450.0;

        let window: PistonWindow = WindowSettings::new("MOcean", [width, height])
            .exit_on_esc(false)
            .resizable(true) // Make the window resizable
            .build()
            .unwrap();

        Self {
            window: window,
            width: width,
            height: height,
        }
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
            if let Some(args) = e.resize_args() {
                self.width = args.window_size[0];
                self.height = args.window_size[1];
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                input.press(key, self.width).await;
            }
            if let Some(Button::Keyboard(key)) = e.release_args() {
                input.release(key).await;
            }

            self.window.draw_2d(&e, |c, g, device| {
                let lines: Vec<&str> = input.text.split("\n").collect();
                let line_height = 32.0; // Height of each line of text
                let total_height = lines.len() as f64 * line_height;
                let y_offset = (self.height as f64 - total_height) / 2.0;

                clear(BLACK, g);
                for (i, line) in lines.iter().enumerate() {
                    let y = y_offset + (i as f64 * line_height);
                    let transform = c
                        .transform
                        .trans(self.width as f64 / 2.0 - (line.len() * 10) as f64, y);

                    text::Text::new_color(WHITE, 32)
                        .draw(line, &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                }

                glyphs.factory.encoder.flush(device);
            });
        }
    }
}
