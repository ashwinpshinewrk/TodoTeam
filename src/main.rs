use app::App;
use iced::Application;

mod app;
mod file;
mod message;
mod todo;
mod ui;

pub const WINDOW_HEIGHT: u16 = 400;
pub const WINDOW_WIDTH: u16 = 720;

#[macro_use]
extern crate savefile_derive;

fn main() {
    let settings = iced::Settings {
        window: iced::window::Settings {
            size: iced::Size::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = App::run(settings);
}
