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

impl Game for Heaven {
	type Input = KeyboardAndMouse;
	type LoadingScreen = ();

	fn load(_window: &Window) -> Task<Heaven> {
		Task::succeed(|| Heaven::new())
	}

	fn interact(&mut self, _input: &mut Self::Input, _window: &mut Window) {
		// let kb = input.keyboard();

		// let input_string =
		// 	kb.released_keys.iter().map(|x| x.to_printable()).collect::<String>();

		// self.text_buffer = format!("{}{}", self.text_buffer, input_string);

		// if kb.released_keys.contains(&KeyCode::Back) {
		// 	self.text_buffer =
		// 		self.text_buffer[..cmp::max(self.text_buffer.len() - 1, 0)].to_string();
		// }
	}

	fn update(&mut self, _window: &Window) {
		// if self.tick_count % 60 < 30 {
		// self.blinker = true;
		// } else {
		// self.blinker = false;
		// }
		// self.tick();
	}

	fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);

		// let mut f = Font::from_bytes(frame.gpu(), &PROFONT).unwrap();

		// match self.blinker {
		// 	true => f.add(make_text(
		// 		&format!("{}_", self.text_buffer),
		// 		Point::new(100.0, 100.0),
		// 		60.0,
		// 	)),
		// 	false => f.add(make_text(
		// 		&format!("{}", self.text_buffer),
		// 		Point::new(100.0, 100.0),
		// 		60.0,
		// 	)),
		// }

		let mut f = Label::new()
			.content("the heaven underground")
			.position(Point::new(600.0, 500.0))
			.bounds((800.0, 500.0))
			.size(60.0)
			.make(frame.gpu());

		let mut target = frame.as_target();
		f.draw(&mut target);
	}
}
