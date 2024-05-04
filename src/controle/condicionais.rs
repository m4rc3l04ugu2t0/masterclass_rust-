#[allow(clippy::comparison_chain)]

pub fn example() {
    let x = 10;
    let y = 5;

    if x > y {
        println!("{} é maior que {}", x, y);
    } else if x < y {
        println!("{} é menor que {}", x, y);
    } else {
        println!("{} é igual a {}", x, y);
    }
}