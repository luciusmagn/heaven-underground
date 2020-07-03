use coffee::graphics::{
	Text, Font, Color, Point, HorizontalAlignment, VerticalAlignment, Gpu,
};

#[derive(Debug, Clone)]
pub struct Label<'a> {
	pub content:              &'a str,
	pub position:             Point,
	pub size:                 f32,
	pub color:                Color,
	pub bounds:               (f32, f32),
	pub horizontal_alignment: HorizontalAlignment,
	pub vertical_alignment:   VerticalAlignment,
	pub font:                 &'a [u8],
}

impl<'a> Label<'a> {
	pub fn new() -> Label<'a> {
		Label {
			content:              "",
			position:             Point::new(0.0, 0.0),
			size:                 20.0,
			color:                Color::RED,
			bounds:               (400.0, 400.0),
			horizontal_alignment: HorizontalAlignment::Left,
			vertical_alignment:   VerticalAlignment::Top,
			font:                 include_bytes!("./ProFontExtended.ttf"),
		}
	}

	pub fn content(mut self, src: &'a str) -> Label<'a> {
		self.content = src;
		self
	}

	pub fn position(mut self, pos: Point) -> Label<'a> {
		self.position = pos;
		self
	}

	pub fn size(mut self, size: f32) -> Label<'a> {
		self.size = size;
		self
	}

	pub fn color(mut self, col: Color) -> Label<'a> {
		self.color = col;
		self
	}

	pub fn bounds(mut self, bounds: (f32, f32)) -> Label<'a> {
		self.bounds = bounds;
		self
	}

	pub fn horiz(mut self, align: HorizontalAlignment) -> Label<'a> {
		self.horizontal_alignment = align;
		self
	}

	pub fn vert(mut self, align: VerticalAlignment) -> Label<'a> {
		self.vertical_alignment = align;
		self
	}

	pub fn make(&mut self, gpu: &mut Gpu) -> Font {
		let mut f =
			Font::from_bytes(gpu, unsafe { std::mem::transmute(self.font) }).unwrap();

		let Label {
			content,
			position,
			size,
			color,
			bounds,
			horizontal_alignment,
			vertical_alignment,
			..
		} = self.clone();
		f.add(Text {
			content,
			position,
			size,
			color,
			bounds,
			horizontal_alignment,
			vertical_alignment,
		});

		f
	}
}
