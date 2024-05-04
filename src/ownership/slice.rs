// Os Slice permitem fazer referência a uma sequência contínua
// de elementos de uma coleção, em vez de á coleção inteira.
// O Slice é uma espécie de referência, por tanto não possui ownership

pub fn example() {
    let text = String::from("Rust é uma linguagem de programação moderna");

    let word = first_word_with('l', &text);
    println!("A palavra é: {}", word);
} 

// Primeira palavra iniciada pela letra informada
fn first_word_with(letter: char, slice: &str) -> &str {
    let bytes = slice.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == letter as u8 {
            return slice[i..].split_whitespace().next().unwrap();
        }
    }

    slice
}