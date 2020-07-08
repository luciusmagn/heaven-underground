use coffee::{
	Timer, Game,
	load::{Task, Join},
};
use coffee::graphics::{Frame, Window};
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::Font;
use either::Either;

use crate::screens::{Menu, Play, About, Options, ReadStory};

use std::fmt;
use std::sync::Arc;
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Direction {
	Top,
	Down,
	Left,
	Right,
	None,
}

#[derive(Clone)]
pub enum Action {
	ChangeScreen(Screen),
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
		match self {
			Action::ChangeScreen(screen) => {
				heaven.screen = screen.clone();
				Ok(())
			}
			Action::MutateState(fun) => fun(heaven),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Screen {
	//Menu(Menu),
	Menu { buttons: Arc<Vec<(String, Action)>>, selected: usize },
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
			buttons:  Arc::new(vec![
				("play game".into(), Action::ChangeScreen(Screen::play())),
				("read story".into(), Action::ChangeScreen(Screen::read_story())),
				("options".into(), Action::ChangeScreen(Screen::options())),
				("about".into(), Action::ChangeScreen(Screen::about())),
				(
					"quit".into(),
					Action::MutateState(Box::new(|game: &mut Heaven| {
						Ok(game.quit_state = true)
					})),
				),
			]),
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
		match h.screen {
			Screen::Menu { .. } => Menu::from_heaven(h).interact(h, input, window),
			Screen::Play { .. } => Play::from_heaven(h).interact(h, input, window),
			Screen::About { .. } => About::from_heaven(h).interact(h, input, window),
			Screen::Options => Options::from_heaven(h).interact(h, input, window),
			Screen::ReadStory => ReadStory::from_heaven(h).interact(h, input, window),
			_ => Ok(()),
		}
	}

	fn render(h: &mut Heaven, frame: &mut Frame, timer: &Timer) {
		match h.screen {
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
	TimeScreen((String, String, String), Node),
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
	pub event_tree: Tree,
	pub minigames:  HashMap<&'static str, Box<dyn Minigame>>,
	pub tick_count: u64,
	pub quit_state: bool,
	pub held_keys:  Vec<KeyCode>,
	pub fonts:      HashMap<&'static str, Font>,
	pub sprites:    HashMap<&'static str, Vec<u8>>,
	pub screen:     Screen,
}

impl Heaven {
	pub fn new() -> Self {
		Self {
			minigames:  HashMap::new(),
			event_tree: Tree::new(),
			screen:     Screen::menu(),
			sprites:    HashMap::new(),
			quit_state: false,
			held_keys:  vec![],
			tick_count: 0,
			fonts:      HashMap::new(),
		}
	}
}

impl Game for Heaven {
	type Input = KeyboardAndMouse;
	type LoadingScreen = ();

	fn load(_window: &Window) -> Task<Heaven> {
		(
			Font::load_from_bytes(include_bytes!("./ProFontExtended.ttf")),
			Task::succeed(Heaven::new),
		)
			.join()
			.map(|(font, mut heaven)| {
				heaven.fonts.insert("ProFontExtended", font);
				heaven
			})
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
		self.quit_state
	}
}
