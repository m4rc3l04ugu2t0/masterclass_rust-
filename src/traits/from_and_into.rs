use core::hash;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
enum Status {
	Broken(u8),
	Working,
}

impl From<u8> for Status {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::Working,
			c => Self::Broken(c),
		}
	}
}

pub fn execute() {
	let status: Status = 0.into();
	println!("Status = 0.into(); => {:?}", status);

	let status = Status::from(1);
	println!("Status::from(1); => {:?}\n", status);

	println!("String to number conversion:");
	string_to_namber();

	println!("&str to String conversion:");
	str_to_string();
}

#[derive(Debug)]
pub struct MyU32(u32);

impl From<String> for MyU32 {
	fn from(value: String) -> Self {
		let parsed_value = value.parse().unwrap_or(0);
		MyU32(parsed_value)
	}
}
fn string_to_namber() {
	let number: MyU32 = "42".to_string().into();
	println!("{:?}", number);
	let inner_u32 = number.0;
	println!("MyU32(42) => number.0 => {:?}", inner_u32);
}

struct MyString(String);

impl<'a> From<&'a str> for MyString {
	fn from(value: &'a str) -> Self {
		let parsed_value = value.to_string();
		MyString(parsed_value)
	}
}

fn str_to_string() {
	let s: String = "hello".into();
	println!("hello.into(); => {:?}", s);
}

struct MyHashMap<K, V>(HashMap<K, V>);

impl<K: Eq + Hash, V> From<Vec<(K, V)>> for MyHashMap<K, V> {
	fn from(items: Vec<(K, V)>) -> Self {
		MyHashMap(items.into_iter().collect())
	}
}

fn vec_to_hash_map() {
	let vec = vec![("a", 1), ("b", 2)];

	let map: MyHashMap<&'static str, i32> = vec.into();
}
