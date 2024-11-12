#![windows_subsystem = "windows"]

pub mod app;
pub mod dialog;
pub mod thread_safe;
pub mod win_str;
pub mod window;

use std::io::Error;
use app::App;

fn main() -> Result<(), Error> {
    let app = App::new();
    App::run(app)
}