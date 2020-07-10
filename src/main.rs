#![feature(const_fn, const_mut_refs)]
#![allow(clippy::type_complexity, clippy::new_without_default, clippy::unit_arg)]

#[macro_use] extern crate lazy_static;

use coffee::{Game, Result, graphics::WindowSettings};

mod keys;

pub mod state;
use state::Heaven;

pub mod text;
pub mod event;
pub mod input;
pub mod screens;

fn main() -> Result<()> {
	Heaven::run(WindowSettings {
		title:      String::from("The Heaven Underground"),
		size:       (1280, 1024),
		resizable:  false,
		fullscreen: false,
		maximized:  false,
	})
}
