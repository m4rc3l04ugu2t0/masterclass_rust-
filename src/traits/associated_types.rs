trait Compute {
	type Target;
	fn compute(&self, rhs: Self::Target) -> Self::Target;
}

struct Add(i32);
struct Sub(f32);

impl Compute for Add {
	type Target = i32;
	fn compute(&self, rhs: Self::Target) -> Self::Target {
		self.0 + rhs
	}
}

impl Compute for Sub {
	type Target = f32;
	fn compute(&self, rhs: Self::Target) -> Self::Target {
		self.0 + rhs
	}
}

pub fn execute() {
	let add = Add(1);
	let two = add.compute(1);
	println!("add.compute => {}\n", two);

	let sub = Sub(1.0);
	let zero = sub.compute(1.0);
	println!("sub.compute => {}", zero);
}
