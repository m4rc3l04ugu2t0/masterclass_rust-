pub fn example() {
    let active = true;
    println!("Booleano: {}", active);

    let character = 'a';
    println!("Charactere: {}", character);

    let name = "Marcelo";
    println!("&str: {}", name);

    let mut name = String::from("Monkey");
    name.push_str(" D. Luffy");
    println!("String: {}", name);

    // i8, i16, i32, i64, i128, usize
    // u8, u16, u32, u64, u128, usize
    let amount = 10;
    println!("Inteiro: {}", amount);

    // f32, f64
    let  price = 10.99;
    println!("Ponto flutuante: {}", price);

}