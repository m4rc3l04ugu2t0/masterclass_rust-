mod controle;
mod funcoes;
mod fundamentos;
mod generics;
mod operator_overloading;
mod ownership;
mod tipos;
mod traits;
mod utils;
use std::process::exit;

use utils::terminal::{clear_scream, show_menu};

fn main() {
	loop {
		let items = [
			"Fundamentos",
			"Tipos",
			"Controle",
			"Funções",
			"Ownership",
			"Traits",
			"Generics",
			"Operator Overloading",
		];
		let selected = show_menu("Principal", &items, true);

		clear_scream();

		match selected {
			1 => fundamentos::execute(),
			2 => tipos::execute(),
			3 => controle::execute(),
			4 => funcoes::execute(),
			5 => ownership::execute(),
			6 => traits::execute(),
			7 => generics::execute(),
			8 => operator_overloading::execute(),
			_ => exit(0),
		}
	}
}
