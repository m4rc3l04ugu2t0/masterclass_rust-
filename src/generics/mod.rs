pub mod example1;
pub mod example2;

use crate::utils::terminal::{clear_scream, press_enter, show_menu};

pub fn execute() {
	loop {
		let items = ["First axample generics", "Second example generics"];

		let selected = show_menu("Generics", &items, false);

		clear_scream();

		match selected {
			1 => example1::example(),
			2 => example2::example(),
			_ => break,
		};

		press_enter();
	}
}
