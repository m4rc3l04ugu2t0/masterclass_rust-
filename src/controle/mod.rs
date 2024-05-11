use crate::utils::terminal::{clear_scream, press_enter, show_menu};
mod condicionais;
mod loops;

pub fn execute() {
	loop {
		let items = [
			"Condicionais",
			"For - Range",
			"For - Array",
			"While",
			"Loop",
		];

		let selected = show_menu("Controle", &items, false);

		clear_scream();

		match selected {
			1 => condicionais::example(),
			2 => loops::example_for_range(),
			3 => loops::example_for_array(),
			4 => loops::example_while(),
			5 => loops::example_while(),
			6 => loops::example_loop(),
			_ => break,
		}

		press_enter();
	}
}
