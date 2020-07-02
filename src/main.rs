use coffee::graphics::{Color, Frame, Window, WindowSettings, Text, Font, Point, HorizontalAlignment, VerticalAlignment};
use coffee::load::Task;
use coffee::input::KeyboardAndMouse;
use coffee::{Game, Result, Timer};

static PROFONT: &[u8] = include_bytes!("./ProFontExtended.ttf");

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("A caffeinated game"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct MyGame {
    // Your game state and assets go here...
    blinker: bool,
    tick_count: u64,
    text_buffer: String,
}

impl MyGame {
	pub fn tick(&mut self) {
		self.tick_count += 1;
	}
}

fn make_text<'a>(src: &'a str, pos: Point, size: f32) -> Text<'a> {
	Text {
		content: src,
		position: pos,
		size: size,
		color: Color::WHITE,
		bounds: (200.0, 200.0),
		horizontal_alignment: HorizontalAlignment::Left,
		vertical_alignment: VerticalAlignment::Top,
	}
}

impl Game for MyGame {
    type Input = KeyboardAndMouse; // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| MyGame { blinker: false, tick_count: 0, text_buffer: String::new() })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
	    let kb = input.keyboard();
	    
    }

    fn update(&mut self, _window: &Window) {
	    if self.tick_count % 60 < 30 {
		    self.blinker = true;
	    } else {
		    self.blinker = false;
	    }
	    self.tick();
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

		let mut f = Font::from_bytes(frame.gpu(), &PROFONT).unwrap();

		match self.blinker {
			true => f.add(make_text("hello world!_", Point::new(100.0, 100.0), 20.0)),
			false => f.add(make_text("hello world!", Point::new(100.0, 100.0), 20.0)),
		}

        let mut target = frame.as_target();
		f.draw(&mut target);

        // Draw your game here. Check out the `graphics` module!
    }
}
