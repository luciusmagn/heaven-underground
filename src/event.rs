use coffee::{
	Timer, Game,
	graphics::{
		Frame,
		Window,
	},
	input::KeyboardAndMouse,
};
use either::Either;

use std::sync::Arc;

use crate::state::{Action, Heaven};

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

#[allow(unused_variables)]
impl Event {
	pub fn render(heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {

	}

	pub fn interact(heaven: &mut Heaven, input: &mut KeyboardAndMouse, window: &mut Window) -> Result<(), Box<dyn std::error::Error + 'static>> {
		Ok(())
	}

	pub fn update(&mut self, window: &Window) {

	}
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
