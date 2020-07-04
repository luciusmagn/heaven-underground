use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window, Color, Point};

use crate::text::Label;
use crate::state::{Screen, Heaven};

pub struct Options;
impl Options {
	pub fn from_heaven(heaven: &Heaven) -> Options {
		if let Screen::Options = heaven.data.screen {
			Options
		} else {
			unreachable!("you are retarded")
		}
	}

	pub fn render(&self, _heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);

		let mut f = Label::new()
			.content("options")
			.position(Point::new(40.0, 30.0))
			.bounds((800.0, 500.0))
			.size(40.0)
			.make(frame.gpu());

		let mut target = frame.as_target();
		f.draw(&mut target);
	}

	pub fn interact(
		self,
		heaven: &mut Heaven,
		input: &mut KeyboardAndMouse,
		_window: &mut Window,
	) -> Result<(), Box<dyn std::error::Error + 'static>> {
		let kb = input.keyboard();

		if kb.is_key_pressed(KeyCode::Back) {
			heaven.data.screen = Screen::menu();
		}

		Ok(())
	}
}
