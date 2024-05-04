use std::io::Write;

use rpassword::prompt_password;

fn show_items(items: &[&str]) {
    for (index, item) in items.iter().enumerate() {
        println!("{} - {}", index + 1, item);
    }
}

pub fn show_menu(tile: &str, items: &[&str], exit: bool) -> u32 {
    clear_scream();

    let complete = String::from("Masterclass Rust :: ") + tile;
    println!("{}", complete);
    println!("{}", String::from("=").repeat(complete.len()));

    show_items(items);

    println!("{}", if exit { "* - Sair" } else { "* - Voltar" });
    print!("\nEscolha uma opção: ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let option: Result<u32, _> = line.trim().parse();

    match option {
        Ok(option) => option,
        _ => 0,
    }
}

pub fn press_enter() {
    prompt_password("Pressione ENTER!").unwrap();
}

pub fn clear_scream() {
    print!("{esc}c", esc = 27 as char);
}