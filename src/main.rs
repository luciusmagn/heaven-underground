use coffee::{
	Game, Result, Timer,
	load::Task,
	input::KeyboardAndMouse,
	graphics::{Color, Frame, Window, WindowSettings, Point},
};

mod keys;

pub mod state;
use state::Heaven;

pub mod text;
use text::Label;

fn main() -> Result<()> {
	Heaven::run(WindowSettings {
		title:      String::from("A caffeinated game"),
		size:       (1280, 1024),
		resizable:  false,
		fullscreen: false,
		maximized:  false,
	})
}
