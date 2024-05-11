use std::fmt::format;

trait Notify {
	fn notify(&self) -> &str;
}

struct Phone {
	txt: String,
}

struct Email {
	subject: String,
	body: String,
}

impl Notify for Phone {
	fn notify(&self) -> &str {
		&self.txt
	}
}

impl Notify for Email {
	fn notify(&self) -> &str {
		&self.subject
	}
}

pub fn execute() {
	let phone = Phone {
		txt: "Foo".to_owned(),
	};

	let email = Email {
		subject: "my email".to_owned(),
		body: "Bar".to_owned(),
	};

	println!("Traits declare behavior that may be implemented by any structures or enumerations.\n\
     Traits are similar to interfaces in other programming languages.\n");
	println!("phone.notify() => {}", phone.notify());
	println!("email.notify() => {}\n", email.notify());
	example2();
}

trait Describable {
	fn describe(&self) -> String;
}

impl Describable for i32 {
	fn describe(&self) -> String {
		format!("This is a number => {}", self)
	}
}

impl Describable for String {
	fn describe(&self) -> String {
		format!("This is a string => {}", self)
	}
}

fn example2() {
	let number = 42;
	let description = number.describe();
	println!("{}", description);

	let text = String::from("Hello rust");
	let description = text.describe();
	println!("{}", description);
}
