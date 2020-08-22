use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window, Color, Point};

use crate::text::Label;
use crate::state::Heaven;
use crate::input::{KeyMan, P};

use std::cmp;
use std::sync::Mutex;

lazy_static! {
	static ref INPUT: Mutex<KeyMan<P<P<P<()>>>>> = Mutex::new(KeyMan::<()>::new()
		.pressed(KeyCode::Down, |heaven| {
			let selected = &mut heaven.screen_data.menu_selected;
			let buttons = &mut heaven.screen_data.menu_buttons;
			if *selected + 1 == buttons.len() {
				*selected = 0;
			} else {
				*selected = cmp::min(*selected + 1, buttons.len() - 1);
			}
		})
		.pressed(KeyCode::Up, |heaven| {
			let selected = &mut heaven.screen_data.menu_selected;
			let buttons = &mut heaven.screen_data.menu_buttons;
			let (num, overflowed) = selected.overflowing_sub(1);
			if !overflowed {
				*selected = cmp::max(num, 0);
			} else {
				*selected = buttons.len() - 1;
			}
		})
		.pressed(KeyCode::Return, |heaven| {
			let selected = heaven.screen_data.menu_selected;
			let buttons = heaven.screen_data.menu_buttons.clone();
			let _ = buttons[selected].1.execute(heaven); // TODO do something with this Result
		}));
}

pub struct Menu;

impl Menu {
	pub fn render(heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);
		let f = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();

		f.add(
			Label::new()
				.content("the heaven underground")
				.position(Point::new(600.0, 500.0))
				.bounds((800.0, 500.0))
				.size(60.0)
				.as_text(),
		);

		for (i, (name, _)) in heaven.screen_data.menu_buttons.iter().enumerate() {
			let content = if i == heaven.screen_data.menu_selected {
				format!("> {}", name)
			} else {
				name.into()
			};

			f.add(
				Label::new()
					.content(&content)
					.position(Point::new(950.0, 600.0 + (i as f32) * 60.0))
					.size(40.0)
					.color(if i == heaven.screen_data.menu_selected {
						Color::WHITE
					} else {
						crate::text::RED
					})
					.as_text(),
			);
		}
		let mut target = frame.as_target();
		f.draw(&mut target);
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
