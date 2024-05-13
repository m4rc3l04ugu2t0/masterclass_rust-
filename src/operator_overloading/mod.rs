use crate::utils::terminal::{clear_scream, press_enter, show_menu};
mod example;
mod index;

pub fn execute() {
	loop {
		let items = ["Example", "Index"];

		let selected = show_menu("Operator Overloading", &items, false);

		clear_scream();

		match selected {
			1 => example::execute(),
			2 => index::execute(),
			_ => break,
		}

		press_enter();
	}
}
