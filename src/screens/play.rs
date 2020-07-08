use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window, Color, Point};

use crate::state::{Screen, Heaven};
use crate::text::{Label, RED, LIGHT_BLUE, DARK_BLUE, YELLOW};

pub struct Play;
impl Play {
	pub fn from_heaven(heaven: &Heaven) -> Play {
		if let Screen::Play { .. } = heaven.screen {
			Play
		} else {
			unreachable!("you are retarded")
		}
	}

	pub fn render(&self, heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);
		let f = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();

		f.add(
			Label::new()
				.content("play game")
				.position(Point::new(40.0, 30.0))
				.bounds((800.0, 500.0))
				.size(40.0)
				.as_text(),
		);

		f.add(
			Label::new()
				.content("bílá")
				.color(Color::WHITE)
				.position(Point::new(300.0, 200.0))
				.bounds((800.0, 500.0))
				.size(60.0)
				.as_text(),
		);
		f.add(
			Label::new()
				.content("červená")
				.color(RED)
				.position(Point::new(300.0, 300.0))
				.bounds((800.0, 500.0))
				.size(60.0)
				.as_text(),
		);
		f.add(
			Label::new()
				.content("světle modrá")
				.color(LIGHT_BLUE)
				.position(Point::new(300.0, 400.0))
				.bounds((800.0, 500.0))
				.size(60.0)
				.as_text(),
		);
		f.add(
			Label::new()
				.content("tmavě modrá")
				.color(DARK_BLUE)
				.position(Point::new(300.0, 500.0))
				.bounds((800.0, 500.0))
				.size(60.0)
				.as_text(),
		);
		f.add(
			Label::new()
				.content("žlutá")
				.color(YELLOW)
				.position(Point::new(300.0, 600.0))
				.bounds((800.0, 500.0))
				.size(60.0)
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

		if kb.is_key_pressed(KeyCode::Back) {
			heaven.screen = Screen::menu();
		}

		Ok(())
	}
}
