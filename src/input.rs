use coffee::input::keyboard::{Keyboard, KeyCode};
use crate::state::Heaven;

pub struct Keys(Vec<KeyCode>);
pub struct KeyTuple<'a>(&'a Keyboard, &'a mut Vec<KeyCode>);
pub struct KeyMan<'a, T>(&'a mut Vec<KeyCode>, T);

pub struct Pressed<T>(KeyCode, fn(&mut Heaven) -> (), T);
pub struct Held<T>(KeyCode, fn(&mut Heaven) -> (), T);
pub struct Released<T>(KeyCode, fn(&mut Heaven) -> (), T);

pub trait KeyInput {
	fn resolve(&self, keyman: &KeyTuple, heaven: &mut Heaven);
}

impl KeyInput for () {
	fn resolve(&self, _: &KeyTuple, _: &mut Heaven) {}
}

impl<T: KeyInput> KeyInput for Pressed<T> {
	fn resolve(&self, keyman: &KeyTuple, heaven: &mut Heaven) {
		let Pressed (code, fun, next) = self;

		if keyman.0.is_key_pressed(*code) && !keyman.1.contains(&code) {
			fun(heaven);
		}

		next.resolve(keyman, heaven)
	}
}

impl<T: KeyInput> KeyInput for Held<T> {
	fn resolve(&self, keyman: &KeyTuple, heaven: &mut Heaven) {
		let Held (code, fun, next) = self;

		if keyman.0.is_key_pressed(*code) && keyman.1.contains(&code) {
			fun(heaven);
		}

		next.resolve(keyman, heaven)
	}
}

impl<T: KeyInput> KeyInput for Released<T> {
	fn resolve(&self, keyman: &KeyTuple, heaven: &mut Heaven) {
		let Released (code, fun, next) = self;

		if keyman.0.was_key_released(*code) {
			fun(heaven);
		}

		next.resolve(keyman, heaven)
	}
}

impl Keys {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn input<'a>(&'a mut self) -> KeyMan<'a, ()> {
		KeyMan(&mut self.0, ())
	}
}

impl<'a, T> KeyMan<'a, T>
	where T: KeyInput,
{
	pub fn pressed(self, key: KeyCode, fun: fn(&mut Heaven) -> ()) -> KeyMan<'a, Pressed<T>> {
		let KeyMan(keys, next) = self;

		KeyMan(keys, Pressed(key, fun, next))
	}

	pub fn held(self, key: KeyCode, fun: fn(&mut Heaven) -> ()) -> KeyMan<'a, Held<T>> {
		let KeyMan(keys, next) = self;

		KeyMan(keys, Held(key, fun, next))
	}

	pub fn released(self, key: KeyCode, fun: fn(&mut Heaven) -> ()) -> KeyMan<'a, Released<T>> {
		let KeyMan(keys, next) = self;

		KeyMan(keys, Released(key, fun, next))
	}

	pub fn execute(mut self, kb: &'a Keyboard, heaven: &mut Heaven) -> () {
		let keys = &mut self.0;
		let tuple: KeyTuple = KeyTuple(kb, keys);

		self.1.resolve(&tuple, heaven);

		for key in &kb.pressed_keys {
			if !keys.contains(key) {
				keys.push(*key);
			}
		}
	}
}

#[allow(invalid_value)]
pub fn compose_test() {
	let mut keys = Keys::new();
	let kb: Keyboard = unsafe { std::mem::zeroed() };
	let mut h: Heaven = unsafe { std::mem::zeroed() };

	keys.input()
		.pressed(KeyCode::A, |_| ())
		.held(KeyCode::A, |_| ())
		.released(KeyCode::A, |_| ())
		.execute(&kb, &mut h);
}
