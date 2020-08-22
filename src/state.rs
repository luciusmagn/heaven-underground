use coffee::{
	Timer, Game,
	load::{Task, Join, loading_screen::ProgressBar},
};
use coffee::graphics::{Frame, Window, CursorIcon};
use coffee::input::{KeyboardAndMouse, keyboard::KeyCode};
use coffee::graphics::Font;

use crate::event::Tree;
use crate::screens::{Menu, Play, About, Options, ReadStory};

use std::fmt;
use std::sync::Arc;
use std::boxed::Box;
use std::collections::HashMap;

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
	Menu,
	About,
	ReadStory,
	Options,
	Quit,
	Play,
	PlayCutscene,
	PlayMinigame,
}

#[derive(Debug, Clone)]
pub struct ScreenData {
	pub menu_buttons:  Arc<Vec<(String, Action)>>,
	pub menu_selected: usize,
	pub cutscene:      String,
	pub minigame:      String,
}

impl Screen {
	/*
	 ** I N I T I A L I Z E R S
	 */
	pub fn menu() -> Screen {
		Screen::Menu
	}

	pub fn play() -> Screen {
		Screen::Play
	}

	pub fn about() -> Screen {
		Screen::About
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
			Screen::Menu => Menu::interact(h, input, window),
			Screen::Play => Play::interact(h, input, window),
			Screen::About => About::interact(h, input, window),
			Screen::Options => Options::interact(h, input, window),
			Screen::ReadStory => ReadStory::interact(h, input, window),
			_ => Ok(()),
		}
	}

	fn render(h: &mut Heaven, frame: &mut Frame, timer: &Timer) {
		match h.screen {
			Screen::Menu => Menu::render(h, frame, timer),
			Screen::Play => Play::render(h, frame, timer),
			Screen::About => About::render(h, frame, timer),
			Screen::Options => Options::render(h, frame, timer),
			Screen::ReadStory => ReadStory::render(h, frame, timer),
			_ => (),
		}
	}

	fn update(h: &mut Heaven, window: &Window) {
		match h.screen {
			Screen::Menu => Menu::update(h, window),
			Screen::Play => Play::update(h, window),
			Screen::About => About::update(h, window),
			Screen::Options => Options::update(h, window),
			Screen::ReadStory => ReadStory::update(h, window),
			_ => (),
		}
	}
}

pub trait Minigame {
	fn update(&mut self, window: &Window);
	fn draw(&mut self, frame: &mut Frame, timer: &Timer);
	fn interact(&mut self, input: &mut KeyboardAndMouse, window: &mut Window);
}

pub struct Heaven {
	pub event_tree:  Tree,
	pub minigames:   HashMap<&'static str, Box<dyn Minigame>>,
	pub tick_count:  u64,
	pub quit_state:  bool,
	pub held_keys:   Vec<KeyCode>,
	pub fonts:       HashMap<&'static str, Font>,
	pub sprites:     HashMap<&'static str, Vec<u8>>,
	pub screen:      Screen,
	pub screen_data: ScreenData,
}

impl Heaven {
	pub fn new() -> Self {
		Self {
			minigames:   HashMap::new(),
			event_tree:  Tree::new(),
			sprites:     HashMap::new(),
			quit_state:  false,
			held_keys:   vec![],
			tick_count:  0,
			fonts:       HashMap::new(),
			screen:      Screen::menu(),
			screen_data: ScreenData {
				menu_buttons:  Arc::new(vec![
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
				menu_selected: 0,
				cutscene:      String::new(),
				minigame:      String::new(),
			},
		}
	}
}

impl Game for Heaven {
	type Input = KeyboardAndMouse;
	type LoadingScreen = ProgressBar;

	fn load(_window: &Window) -> Task<Heaven> {
		(
			Font::load_from_bytes(include_bytes!("../fonts/ProFontExtended.ttf")),
			Font::load_from_bytes(include_bytes!("../fonts/Bagnard.ttf")),
			Task::succeed(Heaven::new),
		)
			.join()
			.map(|(profont, bagnard, mut heaven)| {
				heaven.fonts.insert("ProFontExtended", profont);
				heaven.fonts.insert("Bagnard", bagnard);
				heaven
			})
	}

	fn interact(&mut self, input: &mut Self::Input, window: &mut Window) {
		if let Err(e) = Screen::interact(self, input, window) {
			eprintln!("{}", e)
		}
	}

	fn update(&mut self, window: &Window) {
		Screen::update(self, window)
	}

	fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
		Screen::render(self, frame, timer)
	}

	fn is_finished(&self) -> bool {
		self.quit_state
	}

	fn cursor_icon(&self) -> CursorIcon {
		CursorIcon::Hidden
	}
}
