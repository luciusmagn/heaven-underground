use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window, Color, Point};

use crate::text::Label;
use crate::input::{KeyMan, P};
use crate::state::{Screen, Heaven};

use std::sync::Mutex;

pub struct Options;

lazy_static! {
	static ref INPUT: Mutex<KeyMan<P<()>>> =
		Mutex::new(KeyMan::<()>::new().pressed(KeyCode::Back, |heaven| {
			heaven.screen = Screen::menu();
			heaven.screen_data.menu_selected = 2;
		}));
}

impl Options {
	pub fn from_heaven(heaven: &Heaven) -> Options {
		if let Screen::Options = heaven.screen {
			Options
		} else {
			unreachable!("you are retarded")
		}
	}

	pub fn render(&self, heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);
		let f = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();

		f.add(
			Label::new()
				.content("options")
				.position(Point::new(40.0, 30.0))
				.bounds((800.0, 500.0))
				.size(40.0)
				.as_text(),
		);

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
		let mut input = INPUT.lock()?;
		input.execute(&kb, heaven);

		Ok(())
	}
}
