use coffee::{
	Timer,
	graphics::{Frame, Window, Point},
	input::KeyboardAndMouse,
};
use either::Either;

use std::sync::Arc;

use crate::text::{Label, WHITE};
use crate::state::{Action, Heaven};

pub type Node = Either<Arc<Event>, Action>;

pub enum Event {
	TimeScreen((String, String, String), usize),
	Text(String, usize),
	//MultipleChoice(Vec<(String, usize)>),
	Choice(String, (String, usize), (String, usize)),
	//GameOver(String, usize),
	//Cutscene(String, usize),
	//Minigame(String, Box<dyn Fn(String) -> Node>),
	//ChapterDelimiter(usize),
	End,
}

#[allow(unused_variables)]
impl Event {
	pub fn render(heaven: &mut Heaven, frame: &mut Frame, _timer: &Timer) {
		use Event::*;

		match &heaven.event_tree.events[heaven.event_tree.position] {
			TimeScreen((line1, line2, line3), _) => {
				let bagnard = &mut heaven.fonts.get_mut("Bagnard").unwrap();

				bagnard.add(
					Label::new()
						.content(&line1.to_uppercase())
						.position(Point::new(100.0, 800.0))
						.size(50.0)
						.bounds((500.0, 500.0))
						.color(WHITE)
						.as_text(),
				);

				let mut target = frame.as_target();
				bagnard.draw(&mut target);
			}
			Text(text, _) => {}
			Choice(text, (first, _), (second, _)) => {}
			End => {
				let profont = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();

				profont.add(
					Label::new()
						.content("this is it for now,")
						.position(Point::new(100.0, 300.0))
						.color(WHITE)
						.bounds((900.0, 500.0))
						.size(90.0)
						.as_text(),
				);

				profont.add(
					Label::new()
						.content("fuck off")
						.position(Point::new(100.0, 400.0))
						.color(WHITE)
						.bounds((900.0, 500.0))
						.size(90.0)
						.as_text(),
				);

				let mut target = frame.as_target();
				profont.draw(&mut target);
			}
		}
	}

	pub fn interact(
		heaven: &mut Heaven,
		input: &mut KeyboardAndMouse,
		window: &mut Window,
	) -> Result<(), Box<dyn std::error::Error + 'static>> {
		Ok(())
	}

	pub fn update(&mut self, window: &Window) {}
}

pub struct Tree {
	pub path:            Vec<usize>,
	//	pub minigame_results: Vec<String>,
	pub events:          Vec<Event>,
	pub position:        usize,
	pub selected_choice: usize,
}

impl Tree {
	pub fn new() -> Self {
		Self {
			path:            vec![],
			//			minigame_results: vec![],
			events:          {
				let mut v = Vec::new();

				v.push(Event::TimeScreen(
					("may, year X".into(), "korubon".into(), "11:18 pm".into()),
					v.len(),
				));

				v.push(Event::Text(
					"this is my fault. i did this to her. and now she's back.".into(),
					v.len(),
				));

				v.push(Event::Text(
					"???: come back!!! neiro, come back you bitch!!!".into(),
					v.len(),
				));

				v.push(Event::Text("shit, shit, shit.".into(), v.len()));

				v.push(Event::Text("???: i'll fucking kill you!!!".into(), v.len()));

				v.push(Event::Text("shit, it's cold out here.".into(), v.len()));

				v.push(Event::Text(
					"where should i even go now? i ruined everything...".into(),
					v.len(),
				));

				v.push(Event::Text("definitely far away from here.".into(), v.len()));

				v.push(Event::Choice(
					"but which way?".into(),
					("to the left, towards...?".into(), v.len()),
					("to the right, towards the centre of korubon".into(), v.len() + 1),
				));

				v.push(
					Event::Text(
						"no, someone would recognize me from the orphanage and bring me back…\
						 it's a matter of time until i’d be face to face with her again".into(),
						v.len() + 1,
					)
				);

				v.push(
					Event::Text(
						"i've never been there... i think it leads out of korubon, for sure, but where to?".into(),
						v.len() + 1,
					)
				);

				v.push(Event::Text("let's go left instead".into(), v.len() - 1));

				v.push(Event::Text("doesn't matter now.".into(), v.len()));

				v.push(Event::Text(
					"i've been running until i couldnt... then i continued to walk...\
						it's been at least an hour.\
						how come not a single car has passed me by?"
						.into(),
					v.len(),
				));

				v.push(
					Event::Text(
						"i'm so angry at myself... i messed everything up... i ruined my life but also her life...\
						shit, i don't even deserve to live anymore! i-".into(),
						v.len())
				);

				v
			},
			position:        0,
			selected_choice: 0,
		}
	}
}
