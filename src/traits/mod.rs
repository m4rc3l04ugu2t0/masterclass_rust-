use crate::utils::terminal::{clear_scream, press_enter, show_menu};

mod associated_types;
mod default;
mod from_and_into;
mod trait_objects;
mod traits;

pub fn execute() {
	loop {
		let items = [
			"Traits",
			"Associated Types",
			"Trait Objects",
			"Default",
			"From/Into",
		];

		let selected = show_menu("Traits", &items, false);
		println!("{}", selected);

		clear_scream();

		match selected {
			1 => traits::execute(),
			2 => associated_types::execute(),
			3 => trait_objects::execute(),
			4 => default::execute(),
			5 => from_and_into::execute(),
			_ => break,
		};

		press_enter();
	}
}
