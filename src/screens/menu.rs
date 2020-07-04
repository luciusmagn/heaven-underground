use coffee::Timer;
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::{Frame, Window, Color, Point};

use crate::text::Label;
use crate::state::{Screen, Action, Heaven};

use std::cmp;

pub struct Menu {
	buttons:  Vec<(String, Action)>,
	selected: usize,
}

impl Menu {
	pub fn from_heaven(heaven: &Heaven) -> Menu {
		if let Screen::Menu { buttons, selected } = &heaven.data.screen {
			Menu { buttons: buttons.clone(), selected: *selected }
		} else {
			unreachable!("you are retarded")
		}
	}

	pub fn render(&self, _heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);

		let mut f = Label::new()
			.content("the heaven underground")
			.position(Point::new(600.0, 500.0))
			.bounds((800.0, 500.0))
			.size(60.0)
			.make(frame.gpu());

		let mut target = frame.as_target();
		f.draw(&mut target);

		for (i, (name, _)) in self.buttons.iter().enumerate() {
			let content =
				if i == self.selected { format!("> {}", name) } else { name.into() };

			let mut f = Label::new()
				.content(&content)
				.position(Point::new(950.0, 600.0 + (i as f32) * 60.0))
				.size(40.0)
				.color(if i == self.selected { Color::WHITE } else { crate::text::RED })
				.make(frame.gpu());

			let mut target = frame.as_target();
			f.draw(&mut target);
		}
	}

	pub fn interact(
		&self,
		heaven: &mut Heaven,
		input: &mut KeyboardAndMouse,
		_window: &mut Window,
	) -> bool {
		let selected = heaven.data.screen.selected().unwrap();
		let kb = input.keyboard();

		if kb.is_key_pressed(KeyCode::Down)
			&& !heaven.data.held_keys.contains(&KeyCode::Down)
		{
			if self.selected + 1 == self.buttons.len() {
				*selected = 0;
			} else {
				*selected = cmp::min(self.selected + 1, self.buttons.len() - 1);
			}
			heaven.data.held_keys.push(KeyCode::Down);
		}
		if kb.is_key_pressed(KeyCode::Up) && !heaven.data.held_keys.contains(&KeyCode::Up)
		{
			let (num, overflowed) = self.selected.overflowing_sub(1);
			if !overflowed {
				*selected = cmp::max(num, 0);
			} else {
				*selected = self.buttons.len() - 1;
			}
			heaven.data.held_keys.push(KeyCode::Up);
		}

		if kb.was_key_released(KeyCode::Down) {
			heaven.data.held_keys.retain(|x| x != &KeyCode::Down);
		}
		if kb.was_key_released(KeyCode::Up) {
			heaven.data.held_keys.retain(|x| x != &KeyCode::Up);
		}

		if kb.is_key_pressed(KeyCode::Return) {
			return true;
		}
		false
	}
}
