pub fn example() {
    let tupla = (1, "sla", 10.99);
    println!("Tupla => {:?}", tupla);
    println!("Tupla.1 => {:?}", tupla.1);

    let (a, b, c) = tupla;
    println!("Desestruturação da tupla => {}, {}, {}", a, b, c);

    let mut array = [0;5];
    println!("array: {:?}", array);

    array[0] = 1;
    array[1] = 2;
    array[2] = 3;
    println!("array => {:?}", array);
    println!("array[2] => {:?}", array[2]);

    // dynamically size type (DST)
    let mut slice = &array[2..4];
    println!("&array[2..4] => {:?}", slice);

    slice = &array[4..];
    slice.iter().for_each(|i| println!("{}", i));

    let mut vec = Vec::new();
    vec.push(1);
    print!("vec => {:?}", vec);

    let mut vec = vec![1, 2, 3];
    vec.push(10);
    println!("vec[0] => {}", vec[0]);
    println!("vec.pop() => {}", vec.pop().unwrap());
}