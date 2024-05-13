use std::ops::Add;

struct Centimeters(f64);

impl Add<Self> for Centimeters {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}

	// equivalent to the above
	// fn add(self, rhs: Centimeters) -> Centimeters {
	//     Centimeters(self.0 + rhs.0)
	// }
}

fn add_distance(a: Centimeters, b: Centimeters) -> Centimeters {
	a + b
}

pub fn execute() {
	let length = Centimeters(20.0);
	let distance = add_distance(length, Centimeters(10.0));
	println!(
		"add_distance(length, Centimeters(10.0)); => {:?}",
		distance.0
	);
}
