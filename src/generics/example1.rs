// struct MyVec<T> {
// 	inner: Vec<T>,
// }

// struct MyVec<T: std::fmt::Debug> {
// 	inner: Vec<T>,
// }

struct MyVec<T>
where
	T: std::fmt::Debug + std::fmt::Display,
{
	inner: Vec<T>,
}

pub fn example() {
	let num: MyVec<usize> = MyVec {
		inner: vec![1, 2, 3],
	};

	println!("num.inner[0] => {}", num.inner[0]);
}
