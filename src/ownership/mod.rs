use crate::utils::terminal::{clear_scream, show_menu};

mod mecanismo;
mod escopo;
mod referencias;
mod slice;

pub fn execute() {
    loop {
        let items = [
            "Básico",
            "Tempo de Vida",
            "Mover",
            "Clone",
            "Cópia",
            "Movendo a Posse #1",
            "Movendo a Posse #2",
            "Referência Imutável",
            "Referência Mutável #1",
            "Referência Mutável #2",
            "Referência Mutável #3",
            "Referência Pendente",
            "Slice",
        ];

        let selected = show_menu("Ownership", &items, false);

        clear_scream();

        match selected {
            1 => escopo::example_basic(),
            2 => escopo::example_lifetime(),
            3 => escopo::example_move(),
            4 => escopo::example_clone(),
            5 => escopo::example_copy(),
            6 => mecanismo::example_a(),
            7 => mecanismo::example_b(),
            8 => referencias::example_ref_immutable(), 
            9 => referencias::example_ref_mutable_a(), 
            10 => referencias::example_ref_mutable_b(), 
            11 => referencias::example_ref_mutable_c(), 
            12 => referencias::example_ref_pending(), 
            13 => slice::example(), 
            _ => break,
        }
    }
}