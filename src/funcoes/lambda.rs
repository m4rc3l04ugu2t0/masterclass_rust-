pub fn example_map() {
    let base_2: Vec<i32> = (1..=10).map(|n| i32::pow(2, n)).collect();
    println!("{:?}", base_2);
}

pub fn example_filter() {
    let is_prime = |n| -> bool {
        if n < 2 {
            return false;
        }
        
        for i in 2..n {
            if i % 2 == 0 {
                return false;
            }
        }

        true
    };

    let primes: Vec<i32> = (1..=1000)
        .filter(|&n| is_prime(n))
        .filter(|&n| n.to_string().ends_with('7'))
        .collect();

    println!("{:?}", primes);
}