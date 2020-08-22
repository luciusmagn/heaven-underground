use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window};

use crate::event::Event;
use crate::state::{Screen, Heaven};

pub struct Play;
impl Play {
	pub fn from_heaven(heaven: &Heaven) -> Play {
		if let Screen::Play { .. } = heaven.screen {
			Play
		} else {
			unreachable!("you are retarded")
		}
	}

	pub fn render(heaven: &mut Heaven, frame: &mut Frame, timer: &Timer) {
		Event::render(heaven, frame, timer)
	}

	pub fn interact(
		heaven: &mut Heaven,
		input: &mut KeyboardAndMouse,
		window: &mut Window,
	) -> Result<(), Box<dyn std::error::Error + 'static>> {
		let kb = input.keyboard();

		if kb.is_key_pressed(KeyCode::Back) {
			heaven.screen = Screen::menu();
		}

		let _ = Event::interact(heaven, input, window); // TODO

		Ok(())
	}

	pub fn update(heaven: &mut Heaven, window: &Window) {
		Event::update(heaven, window)
	}
}
