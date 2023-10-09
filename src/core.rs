use std::time::Instant;

use piston::WindowSettings;
use piston_window::*;
use regex::Regex;

use crate::{mala::MaLa, widgets::input::Input as WInput};

pub struct Core {
    window: PistonWindow,
    width: f64,
    height: f64,
}

impl Core {
    const BLACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    pub fn new() -> Self {
        let width = 850.0;
        let height = 450.0;

        let window: PistonWindow = WindowSettings::new("MOcean", [width, height])
            .exit_on_esc(false)
            .resizable(true)
            .build()
            .unwrap();

        Self {
            window: window,
            width: width,
            height: height,
        }
    }

    pub async fn run(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let mut glyphs = self.window.load_font(assets.join("font.ttf")).unwrap();

        let mut input = WInput::new().await;
        let mut mala = MaLa::new();

        let mut last_len = 0;

        while let Some(e) = self.window.next() {
            let now = Instant::now();

            if now.duration_since(input.cursor.last_blink) >= input.cursor.blink_interval {
                input.cursor.visibility = !input.cursor.visibility;
                input.cursor.last_blink = now;
            }

            if let Some(args) = e.resize_args() {
                self.width = args.window_size[0];
                self.height = args.window_size[1];
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                input.press(key, self.width).await;
                mala.format(&input.text).await;
            }
            if let Some(Button::Keyboard(key)) = e.release_args() {
                input.release(key).await;
            }

            self.window.draw_2d(&e, |c, g, device| {
                if last_len != input.text.chars().count() {
                    last_len = input.text.chars().count();

                    let lines: Vec<&str> = input.text.lines().collect();
                    let line_height = 32.0;
                    let total_height = lines.len() as f64 * line_height;
                    let y_offset = (self.height as f64 - total_height) / 2.0;

                    clear(Self::BLACK, g);

                    // for (i, line) in lines.iter().enumerate() {
                    //     let y = y_offset + (i as f64 * line_height);
                    //     let x = self.width as f64 / 2.0 - (line.len() * 8) as f64;

                    //     let transform = c.transform.trans(x, y);

                    //     text::Text::new_color(Self::WHITE, line_height as u32)
                    //         .draw(line, &mut glyphs, &c.draw_state, transform, g)
                    //         .unwrap();
                    // }

                    let lines: Vec<&str> = mala.content.lines().collect();

                    for (i, line) in lines.iter().enumerate() {
                        let mut result = "".to_string();
                        let expressions: Vec<&str> = line.split("$)").collect();
                        println!("{:#?}", expressions);

                        for i in 0..expressions.len() {
                            if i + 1 < expressions.len() {
                                let next_exp = expressions.get(i + 1).unwrap();
                                let exp = expressions.get(i).unwrap();

                                /* ---- Power ---- */
                                if exp.contains("}^{") {
                                    let base = exp
                                        .split("{")
                                        .nth(1)
                                        .unwrap()
                                        .replace("}", "")
                                        .replace("^", "");
                                    let exponent = exp.split("^{").nth(1).unwrap().replace("}", "");

                                    result += &base;
                                    for ch in exponent.chars() {
                                        let replacement = match ch {
                                            '1' => "¹",
                                            '2' => "²",
                                            '3' => "³",
                                            '4' => "⁴",
                                            '5' => "⁵",
                                            '6' => "⁶",
                                            '7' => "⁷",
                                            '8' => "⁸",
                                            '9' => "⁹",
                                            '0' => "⁰",
                                            '-' => "⁻",
                                            _ => "",
                                        };
                                        result += replacement;
                                    }
                                }

                                /* ----  TODO: Fractions ---- */

                                /* ---- Text ---- */

                                if !next_exp.is_empty() && !next_exp.starts_with("$") {
                                    result += next_exp;
                                }
                            }
                        }

                        let y = y_offset + (i as f64 * line_height);
                        let x = self.width as f64 / 2.0 - (result.len() * 8) as f64;
                        text::Text::new_color(Self::WHITE, line_height as u32)
                            .draw(
                                &result,
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(x, y),
                                g,
                            )
                            .unwrap();
                    }

                    for (i, line) in lines.iter().enumerate() {
                        let y = y_offset + (i as f64 * line_height) - 200.0;
                        let x = self.width as f64 / 2.0 - (line.len() * 8) as f64;

                        let transform = c.transform.trans(x, y);

                        text::Text::new_color([0.8, 0.8, 0.8, 1.0], 16)
                            .draw(&line, &mut glyphs, &c.draw_state, transform, g)
                            .unwrap();
                    }

                    // let current_line_index = input.text.lines().count().saturating_sub(1);
                    // let cursor_line_index = if current_line_index > 0 {
                    //     current_line_index
                    // } else {
                    //     0
                    // };

                    // if let Some(current_line) = lines.get(cursor_line_index) {
                    //     let y = y_offset + (cursor_line_index as f64 * line_height);
                    //     let x = self.width as f64 / 2.0 - (current_line.len() * 8) as f64;

                    //     if input.cursor.visibility {
                    //         let cursor_x = x + glyphs
                    //             .width(line_height as u32, &current_line)
                    //             .unwrap_or(0.0);

                    //         input.cursor.x = cursor_x + 10.0;
                    //         input.cursor.y = y - line_height;

                    //         rectangle(
                    //             Self::WHITE,
                    //             [input.cursor.x, input.cursor.y, 2.0, line_height],
                    //             c.transform,
                    //             g,
                    //         );
                    //     }
                    // }

                    glyphs.factory.encoder.flush(device);
                }
            });
        }
    }
}
