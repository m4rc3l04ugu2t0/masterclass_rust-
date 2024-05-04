// Em um dado momento, você pode ter uma referência  imutavel
// ou qualquer número de referência imutáveis
// As referências devem sempre ser válidas.

pub fn example_ref_immutable() {
    let text = String::from("Rust é uma linguagem de programação de sistemas 
    que roda incrivelmente rápido, previne falhas de 
    segmentação e garante segurança concorrente.");

    let amount = calc_number_of_words(&text);
    println!("{}", text);
    println!("O texto tem {} palavras", amount);
}

#[allow(clippy::ptr_arg)]
// Recebe uma referência para o texto (não toma posse)
// Borrows (empresta) o texto
fn calc_number_of_words(text: &String) -> usize {
    text.split_whitespace().count()
    // text é descartado (drop não é chamado porque não tem a posse - ownership)
}

pub fn example_ref_mutable_a() {
    let name = String::from("Monkey");
    last_name(&name);
    println!("Nome: {}", name);
}

#[allow(clippy::ptr_arg)]
fn last_name(name: &String) {
    // name.push_str(" D. Luffy");
    println!("name.len() => {}", name.len());
}

pub fn example_ref_mutable_b() {
    let mut name =String::from("Monkey");
    last_name_mutable(&mut name);
    println!("name => {}", name);

    let n1 = &mut name;
    println!("n1 => {}", n1);

    let n2 = &mut name;
    println!("n2 => {}", n2);
}

fn last_name_mutable(name: &mut String) {
    name.push_str("D. Luffy");
}

pub fn example_ref_mutable_c() {
    let mut name = String::from("Sanji");

    // Imutável sem problema
    let _s1 = &name;

    // mutável - prople se troca a ordem
    let _s2 = &mut name;
}

pub fn example_ref_pending() {
    // let points_to_nothing = generate_pending(); 
    // println!("Referência Pendente {}", points_to_nothing);

    let no_pending = not_pending();
    println!("Sem pendência => {}", no_pending);
}

// fn generate_pending() -> &String {
//     let text = String::from("Referência Pendente");
//     &text // retorna uma referência que vai ser descartada
// }

#[allow(clippy::let_and_return)]
fn not_pending() -> String {
    let text = String::from("Sem pendência");
    text // retorna uma referência que será movida para a função chamadora
}