pub mod menu;
pub mod play;
pub mod about;
pub mod options;
pub mod read_story;

pub use self::{
	menu::Menu, play::Play, about::About, options::Options, read_story::ReadStory,
};
