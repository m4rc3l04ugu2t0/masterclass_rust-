#[derive(Debug)]
struct Foo {
	a: usize,
	b: usize,
}

impl Default for Foo {
	fn default() -> Self {
		Self { a: 0, b: 0 }
	}
}

pub fn execute() {
	let foo = Foo::default();

	let foo = Foo {
		a: 10,
		..Default::default()
	};

	let maybe: Option<Foo> = None;
	println!("let maybe: Option<Foo> = None; => {:?}", maybe);

	let definitely = maybe.unwrap_or_default();
	println!("maybe.unwrap_or_default() => {:?}", definitely);
}
