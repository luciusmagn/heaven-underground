use coffee::{Timer, Game, load::Task};
use coffee::graphics::{Frame, Window, Color, Point};
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use either::Either;

use crate::text::Label;

use std::cmp;
use std::fmt;
use std::sync::Arc;
use std::boxed::Box;
use std::collections::HashMap;



#[derive(Debug)]
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

impl fmt::Debug for Action {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Action::ChangeScreen(screen) =>
				f.debug_tuple("Action::ChangeScreen").field(screen).finish(),
			Action::MutateState(_) => write!(f, "MutateState"),
		}
	}
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

#[derive(Debug)]
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
	/*
	 ** M E N U
	 */
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
						Ok(game.data.quit_state = true)
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

	fn interact_menu(
		&mut self,
		held_keys: &mut Vec<KeyCode>,
		input: &mut KeyboardAndMouse,
		_window: &mut Window,
	) -> bool {
		if let Screen::Menu { buttons, selected } = self {
			let kb = input.keyboard();

			if kb.is_key_pressed(KeyCode::Down) && !held_keys.contains(&KeyCode::Down) {
				if *selected + 1 == buttons.len() {
					*selected = 0;
				} else {
					*selected = cmp::min(*selected + 1, buttons.len() - 1);
				}
				held_keys.push(KeyCode::Down);
			}
			if kb.is_key_pressed(KeyCode::Up) && !held_keys.contains(&KeyCode::Up) {
				let (num, overflowed) = selected.overflowing_sub(1);
				if !overflowed {
					*selected = cmp::max(num, 0);
				} else {
					*selected = buttons.len() - 1;
				}
				held_keys.push(KeyCode::Up);
			}

			if kb.was_key_released(KeyCode::Down) {
				held_keys.retain(|x| x != &KeyCode::Down);
			}
			if kb.was_key_released(KeyCode::Up) {
				held_keys.retain(|x| x != &KeyCode::Up);
			}

			if kb.is_key_pressed(KeyCode::Return) {
				return true;
			}
		}
		false
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

#[derive(Debug)]
pub struct GameInfo {
	pub tick_count: u64,
	pub quit_state: bool,
	pub held_keys:  Vec<KeyCode>,
	pub fonts:      HashMap<&'static str, Vec<u8>>,
	pub sprites:    HashMap<&'static str, Vec<u8>>,
	pub screen:     Screen,
}

pub struct Heaven {
	pub event_tree: Tree,
	pub minigames:  HashMap<&'static str, Box<dyn Minigame>>,
	pub data:       GameInfo,
}

impl Heaven {
	pub fn new() -> Self {
		Self {
			minigames:  HashMap::new(),
			event_tree: Tree::new(),
			data:       GameInfo {
				screen:     Screen::menu(),
				sprites:    HashMap::new(),
				quit_state: false,
				held_keys:  vec![],
				tick_count: 0,
				fonts:      {
					let mut h = HashMap::new();
					h.insert(
						"ProFont",
						include_bytes!("./ProFontExtended.ttf").iter().cloned().collect(),
					);
					h
				},
			},
		}
	}
}

impl Game for Heaven {
	type Input = KeyboardAndMouse;
	type LoadingScreen = ();

	fn load(_window: &Window) -> Task<Heaven> {
		Task::succeed(|| Heaven::new())
	}

	fn interact(&mut self, input: &mut Self::Input, window: &mut Window) {
		self.data.quit_state =
			self.data.screen.interact_menu(&mut self.data.held_keys, input, window);
	}

	fn update(&mut self, _window: &Window) {}

	fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
		self.data.screen.render(frame, timer)
	}

	fn is_finished(&self) -> bool {
		self.data.quit_state
	}
}
