use std::ops::Add;

fn sum<T: Add<Output = T>>(lhs: T, rhs: T) -> T {
	lhs + rhs
}

pub fn example() {
	let two = sum(1, 1);
	println!("sum(1, 1); => {}", two);
	let two = sum::<f64>(1.0, 1.0);
	println!("sum::<f64>(1.0, 1.0); => {}", two);
	// let four_ish = sum(2, 2.0);
	// ERROR: 2 is an integer and 2.0 is a floating point number,
	//        but the generic function requires both types be the same
}
