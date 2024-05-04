pub fn immutable() {
    let x = 5;
    // x = 10;
    println!("x => {}", x);
}

pub fn changeable() {
    let mut x = 10;
    let y = x;
    println!("x ,y => {} {}", x, y);

    x = 15;
    println!("x ,y => {} {}", x, y);
}

pub fn constants() {
    const VARIABLE_CONST: u8 = 30;
    println!("const: {}", VARIABLE_CONST);
}

pub fn shadowing() {
    let a = 25;
    println!("Variavel a: {}", a);
    let a = "A";
    println!("Variavel a: {}", a);
    let a = [1, 2, 3, 4, 5];
    println!("Variavel a: {:?}", a);
}