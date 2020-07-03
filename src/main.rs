use coffee::{
	Game, Result,
	graphics::WindowSettings,
};

mod keys;

pub mod state;
use state::Heaven;

pub mod text;

fn main() -> Result<()> {
	Heaven::run(WindowSettings {
		title:      String::from("A caffeinated game"),
		size:       (1280, 1024),
		resizable:  false,
		fullscreen: false,
		maximized:  false,
	})
}
