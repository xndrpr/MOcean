mod widgets;

use core::Core;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod core;

#[tokio::main]
async fn main() {
    let mut core = Core::new();
    core.run().await;
}
