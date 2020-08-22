use coffee::{
	Timer,
	graphics::{Frame, Window, Point},
	input::{KeyboardAndMouse, keyboard::KeyCode},
};
use either::Either;

use std::sync::{Arc, Mutex, Once};

use crate::text::{Label, WHITE, RED, BLACK, YELLOW};
use crate::state::{Action, Heaven};
use crate::input::{KeyMan, P};

pub type Node = Either<Arc<Event>, Action>;

pub enum Event {
	TimeScreen((String, String, String), usize),
	Text(String, String, usize),
	//MultipleChoice(Vec<(String, usize)>),
	Choice(String, String, (String, usize), (String, usize)),
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

		frame.clear(BLACK);

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
				bagnard.add(
					Label::new()
						.content(&line2)
						.position(Point::new(100.0, 860.0))
						.size(50.0)
						.bounds((500.0, 500.0))
						.color(WHITE)
						.as_text(),
				);
				bagnard.add(
					Label::new()
						.content(&line3)
						.position(Point::new(100.0, 920.0))
						.size(50.0)
						.bounds((500.0, 500.0))
						.color(RED)
						.as_text(),
				);

				let mut target = frame.as_target();
				bagnard.draw(&mut target);
			}
			Text(text, emotion, _) => {
				let profont = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();
				profont.add(
					Label::new()
						.content(&text)
						.position(Point::new(100.0, 800.0))
						.color(WHITE)
						.bounds((900.0, 800.0))
						.size(30.0)
						.as_text(),
				);

				profont.add(
					Label::new()
						.position(Point::new(
							650.0f32 + (rand::random::<f32>() * 10.0),
							300.0f32 + (rand::random::<f32>() * 10.0),
						))
						.content(&emotion)
						.color(RED)
						.bounds((600.0, 1000.0))
						.size(65.0)
						.as_text(),
				);

				let mut target = frame.as_target();
				profont.draw(&mut target);
			}
			Choice(text, emotion, (first, _), (second, _)) => {
				let profont = &mut heaven.fonts.get_mut("ProFontExtended").unwrap();

				profont.add(
					Label::new()
						.content(&text)
						.position(Point::new(100.0, 750.0))
						.color(WHITE)
						.bounds((900.0, 800.0))
						.size(45.0)
						.as_text(),
				);

				profont.add(
					Label::new()
						.position(Point::new(
							650.0f32 + (rand::random::<f32>() * 10.0),
							300.0f32 + (rand::random::<f32>() * 10.0),
						))
						.content(&emotion)
						.color(RED)
						.bounds((600.0, 1000.0))
						.size(65.0)
						.as_text(),
				);

				if heaven.event_tree.selected_choice > 1 {
					heaven.event_tree.selected_choice = 0;
				}

				match heaven.event_tree.selected_choice {
					0 => {
						profont.add(
							Label::new()
								.content(&format!("> {}", first))
								.position(Point::new(140.0, 820.0))
								.color(YELLOW)
								.bounds((900.0, 800.0))
								.size(30.0)
								.as_text(),
						);

						profont.add(
							Label::new()
								.content(&second)
								.position(Point::new(140.0, 900.0))
								.color(YELLOW)
								.bounds((900.0, 800.0))
								.size(30.0)
								.as_text(),
						);
					}
					1 => {
						profont.add(
							Label::new()
								.content(&first)
								.position(Point::new(140.0, 820.0))
								.color(YELLOW)
								.bounds((900.0, 800.0))
								.size(30.0)
								.as_text(),
						);

						profont.add(
							Label::new()
								.content(&format!("> {}", second))
								.position(Point::new(140.0, 900.0))
								.color(YELLOW)
								.bounds((900.0, 800.0))
								.size(30.0)
								.as_text(),
						);
					}
					_ => (),
				}

				let mut target = frame.as_target();
				profont.draw(&mut target);
			}
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
		use Event::*;
		let kb = input.keyboard();

		lazy_static! {
			static ref TIMESCREEN_INPUT: Mutex<KeyMan<P<()>>> =
				Mutex::new(KeyMan::<()>::new().pressed(KeyCode::Space, |mut heaven| {
					if heaven.event_tree.cooldown > 0 {
						return;
					}
					if let TimeScreen(_, next) =
						&heaven.event_tree.events[heaven.event_tree.position]
					{
						println!("next: {}", next);
						heaven.event_tree.position = *next;
						heaven.event_tree.cooldown = 40;
					}
				}));
			static ref TEXT_INPUT: Mutex<KeyMan<P<()>>> =
				Mutex::new(KeyMan::<()>::new().pressed(KeyCode::Space, |mut heaven| {
					println!("fired");
					if heaven.event_tree.cooldown > 0 {
						return;
					}
					if let Text(_, _, next) =
						&heaven.event_tree.events[heaven.event_tree.position]
					{
						println!("next: {}", next);
						heaven.event_tree.position = *next;
						heaven.event_tree.cooldown = 40;
					}
				}));
			static ref CHOICE_INPUT: Mutex<KeyMan<P<P<P<()>>>>> = Mutex::new(
				KeyMan::<()>::new()
					.pressed(KeyCode::Space, |mut heaven| {
						println!("selected");
						if heaven.event_tree.cooldown > 0 {
							return;
						}
						if let Choice(_, _, (_, first), (_, second)) =
							&heaven.event_tree.events[heaven.event_tree.position]
						{
							match heaven.event_tree.selected_choice {
								0 => {
									heaven.event_tree.position = *first;
									heaven.event_tree.cooldown = 40;
								}
								1 => {
									heaven.event_tree.position = *second;
									heaven.event_tree.cooldown = 40;
								}
								_ => unreachable!("you dun goofed"),
							}
							heaven.event_tree.selected_choice = 0;
						}
					})
					.pressed(KeyCode::Down, |mut heaven| {
						println!("down");
						heaven.event_tree.selected_choice =
							(heaven.event_tree.selected_choice + 1) % 2;
					})
					.pressed(KeyCode::Up, |mut heaven| {
						println!("up");
						if heaven.event_tree.selected_choice == 0 {
							heaven.event_tree.selected_choice = 1;
						} else {
							heaven.event_tree.selected_choice = 0;
						}
					})
			);
		}

		static START: Once = Once::new();
		START.call_once(|| {
			let a = TIMESCREEN_INPUT.lock().unwrap();
			let b = TEXT_INPUT.lock().unwrap();
			let c = CHOICE_INPUT.lock().unwrap();

			drop(a);
			drop(b);
			drop(c);
		});

		match &heaven.event_tree.events[heaven.event_tree.position] {
			TimeScreen(_, _) => {
				let mut input = TIMESCREEN_INPUT.lock()?;
				input.execute(&kb, heaven);
			}
			Text(_, _, _) => {
				let mut input = TEXT_INPUT.lock()?;
				input.execute(&kb, heaven);
			}
			Choice(_, _, (_, first), (_, second)) => {
				let mut input = CHOICE_INPUT.lock()?;
				input.execute(&kb, heaven);
			}
			End => {}
		}
		Ok(())
	}

	pub fn update(heaven: &mut Heaven, window: &Window) {
		if heaven.event_tree.cooldown > 0 {
			heaven.event_tree.cooldown -= 1;
		}
	}
}

pub struct Tree {
	pub path:            Vec<usize>,
	//	pub minigame_results: Vec<String>,
	pub events:          Vec<Event>,
	pub position:        usize,
	pub selected_choice: usize,
	pub cooldown:        usize,
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
					v.len() + 1,
				));

				v.push(Event::Text("".into(), "*O O O P S*".into(), v.len() + 1));

				v.push(Event::Text(
					"NEIRO: this is my fault. i did this to her. and now she's back."
						.into(),
					"spooked af".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"???: come back!!! neiro, come back you bitch!!!".into(),
					"big angery".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"NEIRO: shit, shit, shit.".into(),
					"turbo spooked dumb-dumb".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"???: i'll fucking kill you!!!".into(),
					"incarnation of anger itself".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"".into(),
					"sonic speed escape boiii".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"NEIRO: shit, it's cold out here.".into(),
					"fucken freezing".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"NEIRO: where should i even go now? i ruined everything...".into(),
					"sad confuse".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"NEIRO: definitely far away from here.".into(),
					"spooked thinkin".into(),
					v.len() + 1,
				));

				v.push(Event::Choice(
					"NEIRO: but which way?".into(),
					"panting and thinkin".into(),
					("to the left, towards...?".into(), v.len() + 2),
					("to the right, towards the centre of korubon".into(), v.len() + 1),
				));

				v.push(
					Event::Text(
						"NEIRO: no, someone would recognize me from the orphanage and bring me back…\
						 it's a matter of time until i’d be face to face with her again".into(),
						 "big-braining".into(),
						v.len() + 2,
					)
				);

				v.push(
					Event::Text(
						"NEIRO: i've never been there... i think it leads out of korubon, for sure, but where to?".into(),
						"curious thonking".into(),
						v.len() + 2,
					)
				);

				v.push(Event::Text(
					"NEIRO: let's go left instead".into(),
					"shame bitch".into(),
					v.len() - 1,
				));

				v.push(Event::Text(
					"NEIRO: doesn't matter now.".into(),
					"going fast".into(),
					v.len() + 1,
				));

				v.push(Event::Text(
					"NEIRO: i've been running until i couldnt... then i continued to walk...\
						it's been at least an hour.\
						how come not a single car has passed me by?"
						.into(),
					"panting".into(),
					v.len() + 1,
				));

				v.push(
					Event::Text(
						"NEIRO: i'm so angry at myself... i messed everything up... i ruined my life but also her life...\
						shit, i don't even deserve to live anymore! i-".into(),
						"self-angery contemplation".into(),
						v.len() + 1)
				);

				v.push(Event::End);

				v
			},
			position:        0,
			selected_choice: 0,
			cooldown:        0,
		}
	}
}
