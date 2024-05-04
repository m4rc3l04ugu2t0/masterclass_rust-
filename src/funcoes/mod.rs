use crate::utils::terminal::{clear_scream, show_menu};
mod lambda;
mod  fns;

pub fn execute() {
    loop {
        let items = [
            "Básicos",
            "Map",
            "Filter",
        ];

        let selected = show_menu("Funções", &items, false);

        clear_scream();

        match selected {
            1 => fns::example(),
            2 => lambda::example_map(),
            3 => lambda::example_filter(),
            _ => break,
        }
    }
}