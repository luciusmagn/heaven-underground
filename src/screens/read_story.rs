use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window, Color, Point, Mesh, Shape, Rectangle};

use crate::text::{Label, RED, YELLOW, LIGHT_BLUE, DARK_BLUE};
use crate::state::{Screen, Heaven};
use crate::input::{KeyMan, P};

use std::sync::Mutex;

pub struct ReadStory;

lazy_static! {
	static ref INPUT: Mutex<KeyMan<P<()>>> =
		Mutex::new(KeyMan::<()>::new().pressed(KeyCode::Back, |heaven| {
			heaven.screen = Screen::menu();
			heaven.screen_data.menu_selected = 1;
		}));
}

impl ReadStory {
	pub fn render(heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);
		let f = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();

		f.add(
			Label::new()
				.content("read story")
				.position(Point::new(40.0, 30.0))
				.bounds((800.0, 500.0))
				.size(40.0)
				.as_text(),
		);

		let mut m = Mesh::new();
		m.fill(
			Shape::Rectangle(Rectangle {
				x:      200.0,
				y:      250.0,
				width:  300.0,
				height: 300.0,
			}),
			RED,
		);
		m.fill(
			Shape::Rectangle(Rectangle {
				x:      520.0,
				y:      250.0,
				width:  300.0,
				height: 300.0,
			}),
			YELLOW,
		);
		m.fill(
			Shape::Rectangle(Rectangle {
				x:      200.0,
				y:      570.0,
				width:  300.0,
				height: 300.0,
			}),
			LIGHT_BLUE,
		);
		m.fill(
			Shape::Rectangle(Rectangle {
				x:      520.0,
				y:      570.0,
				width:  300.0,
				height: 300.0,
			}),
			DARK_BLUE,
		);

		let mut target = frame.as_target();
		f.draw(&mut target);
		m.draw(&mut target);
	}

	pub fn interact(
		heaven: &mut Heaven,
		input: &mut KeyboardAndMouse,
		_window: &mut Window,
	) -> Result<(), Box<dyn std::error::Error + 'static>> {
		let kb = input.keyboard();
		let mut input = INPUT.lock()?;
		input.execute(&kb, heaven);

		Ok(())
	}

	pub fn update(_: &mut Heaven, _: &Window) {}
}
