use coffee::{Timer};
use coffee::graphics::{Frame, Window};
use coffee::input::KeyboardAndMouse;
use either::Either;

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
	ChangeScreen,
	MutateState(Box<dyn Fn(&mut Heaven) -> ()>),
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
		Screen::Menu { buttons: vec![], selected: 0 }
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
		}
	}
}
