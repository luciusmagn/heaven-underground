use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window, Color, Point};

use crate::text::Label;
use crate::state::{Screen, Action, Heaven};

use std::cmp;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Menu {
	buttons:  Arc<Vec<(String, Action)>>,
	selected: usize,
}

impl Menu {
	pub fn from_heaven(heaven: &Heaven) -> Menu {
		if let Screen::Menu { buttons, selected } = &heaven.screen {
			Menu { buttons: buttons.clone(), selected: *selected }
		} else {
			unreachable!("you are retarded")
		}
	}

	pub fn render(&self, heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);
		let f = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();

		f.add(
			Label::new()
				.content("the heaven\n underground")
				.position(Point::new(600.0, 500.0))
				.bounds((800.0, 500.0))
				.size(60.0)
				.as_text(),
		);

		for (i, (name, _)) in self.buttons.iter().enumerate() {
			let content =
				if i == self.selected { format!("> {}", name) } else { name.into() };

			f.add(
				Label::new()
					.content(&content)
					.position(Point::new(950.0, 600.0 + (i as f32) * 60.0))
					.size(40.0)
					.color(if i == self.selected {
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
		&self,
		heaven: &mut Heaven,
		input: &mut KeyboardAndMouse,
		_window: &mut Window,
	) -> Result<(), Box<dyn std::error::Error + 'static>> {
		let selected = heaven.screen.selected().unwrap();
		let kb = input.keyboard();

		if kb.is_key_pressed(KeyCode::Down) && !heaven.held_keys.contains(&KeyCode::Down)
		{
			if self.selected + 1 == self.buttons.len() {
				*selected = 0;
			} else {
				*selected = cmp::min(self.selected + 1, self.buttons.len() - 1);
			}
			heaven.held_keys.push(KeyCode::Down);
		}
		if kb.is_key_pressed(KeyCode::Up) && !heaven.held_keys.contains(&KeyCode::Up) {
			let (num, overflowed) = self.selected.overflowing_sub(1);
			if !overflowed {
				*selected = cmp::max(num, 0);
			} else {
				*selected = self.buttons.len() - 1;
			}
			heaven.held_keys.push(KeyCode::Up);
		}

		if kb.was_key_released(KeyCode::Down) {
			heaven.held_keys.retain(|x| x != &KeyCode::Down);
		}
		if kb.was_key_released(KeyCode::Up) {
			heaven.held_keys.retain(|x| x != &KeyCode::Up);
		}

		if kb.is_key_pressed(KeyCode::Return) {
			return self.buttons[self.selected].1.execute(heaven);
		}
		Ok(())
	}
}
