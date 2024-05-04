pub fn example() {
    fn_basic();
    fn_with_params(10, 20);

    let r = fn_with_return();
    println!("Retorno #1 => {}", r);

    let  r = fn_with_params_and_return(10, 10);
    println!("Retorno #2 => {}", r);
}

pub fn fn_basic() {
    println!("Função básica!");
}

pub fn fn_with_params(a: i32, b: i32) {
    println!("Função com pametros:\n a => {}\n b => {}", a, b);
}

pub fn fn_with_return() -> i32 {
    10
}

pub fn fn_with_params_and_return(a: i32, b: i32) -> i32 {
    a + b
}