use coffee::input::keyboard::KeyCode;

pub trait DisplayKey {
	fn to_printable(&self) -> &'static str;
}

impl DisplayKey for KeyCode {
	fn to_printable(&self) -> &'static str {
		match self {
			KeyCode::A => "a",
			KeyCode::B => "b",
			KeyCode::C => "c",
			KeyCode::D => "d",
			KeyCode::E => "e",
			KeyCode::F => "f",
			KeyCode::G => "g",
			KeyCode::H => "h",
			KeyCode::I => "i",
			KeyCode::J => "j",
			KeyCode::K => "k",
			KeyCode::L => "l",
			KeyCode::M => "m",
			KeyCode::N => "n",
			KeyCode::O => "o",
			KeyCode::P => "p",
			KeyCode::Q => "q",
			KeyCode::R => "r",
			KeyCode::S => "s",
			KeyCode::T => "t",
			KeyCode::U => "u",
			KeyCode::V => "v",
			KeyCode::W => "w",
			KeyCode::X => "x",
			KeyCode::Y => "y",
			KeyCode::Z => "z",
			KeyCode::Space => " ",
			KeyCode::Colon => ":",
			KeyCode::Comma => ",",
			KeyCode::Period => ".",
			KeyCode::Return => "\n",
			_ => "",
		}
	}
}
