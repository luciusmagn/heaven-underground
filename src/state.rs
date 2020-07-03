use coffee::{Timer, Game, load::Task};
use coffee::graphics::{Frame, Window, Color, Point};
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use either::Either;

use crate::text::Label;

use std::cmp;
use std::sync::Arc;
use std::boxed::Box;
use std::collections::HashMap;

pub enum Direction {
	Top,
	Down,
	Left,
	Right,
	None,
}

pub enum Action {
	ChangeScreen(&'static str),
	MutateState(
		Box<dyn Fn(&mut Heaven) -> Result<(), Box<dyn std::error::Error + 'static>>>,
	),
}

impl Action {
	pub fn execute(
		&self,
		heaven: &mut Heaven,
	) -> Result<(), Box<dyn std::error::Error + 'static>> {
		match self {
			Action::ChangeScreen(screen) => match screen {
				_ => Ok(()),
			},
			Action::MutateState(fun) => fun(heaven),
		}
	}
}

pub enum Screen {
	Menu { buttons: Vec<(String, Action)>, selected: usize },
	About { scrolling_dir: Direction },
	ReadStory,
	Options, //?
	Quit,
	Play { buttons: Vec<(String, Action)> },
	PlayCutscene(String),
	PlayMinigame(String),
}

impl Screen {
	fn menu() -> Screen {
		Screen::Menu {
			buttons:  vec![
				("play game".into(), Action::ChangeScreen("play_game")),
				("read story".into(), Action::ChangeScreen("read_story")),
				("options".into(), Action::ChangeScreen("options")),
				("about".into(), Action::ChangeScreen("about")),
				(
					"quit".into(),
					Action::MutateState(Box::new(|game: &mut Heaven| {
						Ok(game.quit_state = true)
					})),
				),
			],
			selected: 0,
		}
	}

	fn render_menu(&self, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);

		let mut f = Label::new()
			.content("the heaven underground")
			.position(Point::new(600.0, 500.0))
			.bounds((800.0, 500.0))
			.size(60.0)
			.make(frame.gpu());

		let mut target = frame.as_target();
		f.draw(&mut target);

		if let Screen::Menu { buttons, selected } = self {
			for (i, (name, _)) in buttons.iter().enumerate() {
				let content =
					if i == *selected { format!("> {}", name) } else { name.into() };

				let mut f = Label::new()
					.content(&content)
					.position(Point::new(950.0, 600.0 + (i as f32) * 60.0))
					.size(40.0)
					.color(if i == *selected { Color::WHITE } else { Color::RED })
					.make(frame.gpu());

				let mut target = frame.as_target();
				f.draw(&mut target);
			}
		}
	}

	fn render(&self, frame: &mut Frame, timer: &Timer) {
		match self {
			Screen::Menu { .. } => self.render_menu(frame, timer),
			_ => (),
		}
	}

	fn interact_menu(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {
		if let Screen::Menu { buttons, ref mut selected } = self {
			let kb = input.keyboard();

			if kb.was_key_released(KeyCode::Down) {
				selected = &mut cmp::max(*selected + 1, buttons.len() - 1)
			}
		}
	}
}

pub type Node = Either<Arc<Event>, Action>;

pub enum Event {
	TimeScreen(Node),
	Text(Node),
	MultipleChoice(Vec<Node>),
	Choice(Node, Node),
	GameOver(String),
	Cutscene(String, Node),
	Minigame(String, Box<dyn Fn(String) -> Node>),
	ChapterDelimiter,
	End,
}

pub struct Tree {
	pub path:             Vec<usize>,
	pub minigame_results: Vec<String>,
	pub start:            Event,
}

impl Tree {
	pub fn new() -> Self {
		Self {
			path:             vec![],
			minigame_results: vec![],
			start:            Event::End,
		}
	}
}

pub trait Minigame {
	fn update(&mut self, window: &Window);
	fn draw(&mut self, frame: &mut Frame, timer: &Timer);
	fn interact(&mut self, input: &mut KeyboardAndMouse, window: &mut Window);
}

pub struct Heaven {
	pub screen:     Screen,
	pub tick_count: u64,
	pub fonts:      HashMap<&'static str, Vec<u8>>,
	pub sprites:    HashMap<&'static str, Vec<u8>>,
	pub event_tree: Tree,
	pub minigames:  HashMap<&'static str, Box<dyn Minigame>>,
	pub quit_state: bool,
}

impl Heaven {
	pub fn new() -> Self {
		Self {
			screen:     Screen::menu(),
			tick_count: 0,
			fonts:      {
				let mut h = HashMap::new();
				h.insert(
					"ProFont",
					include_bytes!("./ProFontExtended.ttf").iter().cloned().collect(),
				);
				h
			},
			sprites:    HashMap::new(),
			event_tree: Tree::new(),
			minigames:  HashMap::new(),
			quit_state: false,
		}
	}
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

	fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
		self.screen.render(frame, timer)
	}
}
