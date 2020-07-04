use coffee::{Timer, Game, load::Task};
use coffee::graphics::{Frame, Window};
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use either::Either;

use crate::screens::{Menu, Play, About, Options, ReadStory};

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

#[derive(Clone)]
pub enum Action {
	ChangeScreen(ScreenName),
	MutateState(Box<fn(&mut Heaven) -> Result<(), Box<dyn std::error::Error + 'static>>>),
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
		use ScreenName::*;

		match self {
			Action::ChangeScreen(screen) => {
				match screen {
					Play => heaven.data.screen = Screen::play(),
					Menu => heaven.data.screen = Screen::menu(),
					About => heaven.data.screen = Screen::about(),
					Options => heaven.data.screen = Screen::options(),
					ReadStory => heaven.data.screen = Screen::read_story(),
					_ => (),
				}

				Ok(())
			}
			Action::MutateState(fun) => fun(heaven),
		}
	}
}

#[derive(Clone, Debug)]
pub enum ScreenName {
	Menu,
	About,
	ReadStory,
	Options,
	Quit,
	Play,
	PlayCutscene,
	PlayMinigame,
}

#[derive(Debug)]
pub enum Screen {
	Menu { buttons: Vec<(String, Action)>, selected: usize },
	About { scrolling_dir: Direction },
	ReadStory,
	Options, //?
	Quit,
	Play,
	PlayCutscene(String),
	PlayMinigame(String),
}

impl Screen {
	/*
	 ** I N I T I A L I Z E R S
	 */
	pub fn menu() -> Screen {
		Screen::Menu {
			buttons:  vec![
				("play game".into(), Action::ChangeScreen(ScreenName::Play)),
				("read story".into(), Action::ChangeScreen(ScreenName::ReadStory)),
				("options".into(), Action::ChangeScreen(ScreenName::Options)),
				("about".into(), Action::ChangeScreen(ScreenName::About)),
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

	pub fn play() -> Screen {
		Screen::Play
	}

	pub fn about() -> Screen {
		Screen::About { scrolling_dir: Direction::Down }
	}

	pub fn options() -> Screen {
		Screen::Options
	}

	pub fn read_story() -> Screen {
		Screen::ReadStory
	}

	/*
		** M A I N
		*/

	fn interact(
		h: &mut Heaven,
		input: &mut KeyboardAndMouse,
		window: &mut Window,
	) -> Result<(), Box<dyn std::error::Error + 'static>> {
		match h.data.screen {
			Screen::Menu { .. } => Menu::from_heaven(h).interact(h, input, window),
			Screen::Play { .. } => Play::from_heaven(h).interact(h, input, window),
			Screen::About { .. } => About::from_heaven(h).interact(h, input, window),
			Screen::Options => Options::from_heaven(h).interact(h, input, window),
			Screen::ReadStory => ReadStory::from_heaven(h).interact(h, input, window),
			_ => Ok(()),
		}
	}

	fn render(h: &mut Heaven, frame: &mut Frame, timer: &Timer) {
		match h.data.screen {
			Screen::Menu { .. } => Menu::from_heaven(h).render(h, frame, timer),
			Screen::Play { .. } => Play::from_heaven(h).render(h, frame, timer),
			Screen::About { .. } => About::from_heaven(h).render(h, frame, timer),
			Screen::Options => Options::from_heaven(h).render(h, frame, timer),
			Screen::ReadStory => ReadStory::from_heaven(h).render(h, frame, timer),
			_ => (),
		}
	}

	/*
		** U T I L S
		*/
	pub fn selected(&mut self) -> Option<&mut usize> {
		if let Screen::Menu { buttons: _, selected } = self {
			Some(selected)
		} else {
			None
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
		if let Err(e) = Screen::interact(self, input, window) {
			eprintln!("{}", e)
		}
	}

	fn update(&mut self, _window: &Window) {}

	fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
		Screen::render(self, frame, timer)
	}

	fn is_finished(&self) -> bool {
		self.data.quit_state
	}
}
