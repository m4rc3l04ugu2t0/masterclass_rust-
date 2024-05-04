// 1. Todo valor em Rust tem um dono.
// 2. Só pode ter um dono por vez.
// 3. Quando dono sair do escopo, valor será descartado.

pub fn example_basic() {
    {
        let s = String::from("Hello");
        println!("s => {}", s);
    } 

    // println!("s => {}", s);
}

pub fn example_lifetime() {
    let x;
    {
        let y = String::from("Escopo interno");
        x = &y;
        println!("x => {}\n y => {}", x, y);
    }
    // println!("{}", x); Error
}

pub fn example_move() {
    // Valor alocado na stack
    let num1 = 10;
    let num2 = num1;
    println!("num1 => {}\n num2 => {}", num1, num2);

    // Valor é alocado na heap
    let s1 = String::from("One Piece");

    // s1 foi movido para s2
    let s2 = s1;

    // println!("{}", s1);
    println!("s2 => {}", s2);
}

pub fn example_clone() {
    let s1 = String::from("Hello Rust!");

    // Clone precisa ser explicitamente chamado
    let s2 = s1.clone();

    println!("s1 => {}\n s2 => {}", s1, s2);
}

pub fn example_copy() {
    // Valores fixos são armazenados na stack e são copiados
    // Precisa implementa o trait Copy
    // i32, f64, bool, char, tuplas com tipos Copy

    let nums_a = [1, 2, 3, 4, 5];
    let nums_b = nums_a;

    println!("nums_a => {:?}\n num_b => {:?}", nums_a, nums_b);
}