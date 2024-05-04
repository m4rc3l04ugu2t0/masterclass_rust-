use crate::utils::terminal::{clear_scream, press_enter, show_menu};

mod basicos;
mod custom;
mod sequencias;

pub fn execute() {
    loop {
        let items = [
            "Básicos",
            "Sequências",
            "Custom - Structs",
            "Custom - Enums",
        ];

        let selected = show_menu("Tipos", &items, false);

        clear_scream();

        match selected {
            1 => basicos::example(),
            2 => sequencias::example(),
            3 => custom::example_enum(),
            4 => custom::example_struct(),
            _ => break,
        }

        press_enter();
    }
}