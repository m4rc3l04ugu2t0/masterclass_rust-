mod primeiros;
mod variaveis;
mod operadores;
use crate::utils::terminal::{show_menu, press_enter, clear_scream};

pub fn execute() {
    loop {
        let items = [
            "Primeiro Exemplo",
            "Variáveis - Imutáveis",
            "Variáveis - Mutáveis",
            "Variáveis - Constantes",
            "Variáveis - Shadowing",
            "Operadores - Aritméticos",
            "Operadores - Relacionais",
            "Operadores - Lógicos",
        ];

        let selected = show_menu("Fundamentos", &items, false);

        clear_scream();

        match selected {
            1 => primeiros::example(),
            2 => variaveis::immutable(),
            3 => variaveis::changeable(),
            4 => variaveis::constants(),
            5 => variaveis::shadowing(),
            6 => operadores::arithmetic(),
            7 => operadores::relational(),
            8 => operadores::logical(),
            _ => break,
        }

        press_enter();
    }
}