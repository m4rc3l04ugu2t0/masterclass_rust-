pub fn example_a() {
    let name = String::from("Luffy");
    show_name(name);

    // println!("{}", name);

    let age = 30;
    show_age(age);

    println!("{}", age);
}

// A referência é movida para o argumento
// que toma posse do valor
fn show_name(name: String) {
    println!("name => {}", name);
} // name pe descartado (drop)

// Recebe uma copia do valor (não toma posse)
fn show_age(age: i32) {
    println!("age => {}", age);
}

pub fn example_b() {
    let name = new_name();
    println!("name => {}", name);

    let (name, t) = calc_length(name);
    println!("O nome {} tem {} letras.", name, t);
}

#[allow(clippy::let_and_return)]
fn new_name() -> String {
    let name = String::from("Zoro");
    name // posse é transferido para a chamada da função  
}

// Recebe a posse do valor (ownership)
fn calc_length(name: String) -> (String, usize) {
    let length = name.len();
    (name, length) // Devolve a posse do valor para a chamada da função
}