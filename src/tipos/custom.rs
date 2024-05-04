use std::fmt;

pub fn example_struct() {
    #[derive(Debug)]
    struct UserRegistered {
        name: String,
        age: u8,
        active: bool,
    }

    impl UserRegistered {
        fn print_data_user() {   
            let user = UserRegistered {
                name: String::from("Luffy"),
                age: 18,
                active: true,
            };
            println!("User => {:?}", user);
            println!(
                "user.name => {},\n\
                user.age => {},\n\
                user.active => {},
            ", user.name, user.age, user.active);
        }
    }

    UserRegistered::print_data_user();
}

pub fn example_enum() {
    enum DaysOfWeek {
        Moday,
        Tuesday,
        Wendnesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl fmt::Display for DaysOfWeek {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DaysOfWeek::Moday => write!(f, "segunda-feira"),
                DaysOfWeek::Tuesday => write!(f, "terça-feira"),
                DaysOfWeek::Wendnesday => write!(f, "quarta-feira"),
                DaysOfWeek::Thursday => write!(f, "quinta-feira"),
                DaysOfWeek::Friday => write!(f, "sexta-feira"),
                DaysOfWeek::Saturday => write!(f, "sábado"),
                DaysOfWeek::Sunday => write!(f, "domingo"),
            }
        }
    }

    println!("DaysOfWeek::Moday => {}", DaysOfWeek::Moday);
    println!("DaysOfWeek::Tuesday{}", DaysOfWeek::Tuesday);
    println!("DaysOfWeek::Wendnesday => {}", DaysOfWeek::Wendnesday);
    println!("DaysOfWeek::Thursday => {}", DaysOfWeek::Thursday);
    println!("DaysOfWeek::Friday => {}", DaysOfWeek::Friday);
    println!("DaysOfWeek::Saturday => {}", DaysOfWeek::Saturday);
    println!("DaysOfWeek::Sunday => {}", DaysOfWeek::Sunday);
}