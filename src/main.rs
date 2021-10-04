

pub fn fibonacci_number_none_recur(n: i32) -> u64 {

	match n {
		0     => panic!("zero is not a right argument to fibonacci_number!"),
		1 | 2 => return 1 as u64,
		_ => {
			let mut a: i32 = 1;
			let mut b: i32 = 1;
			let mut result: i32 = 2;

			for _ in 2..n {
				let placeholder: i32 = b;
				b = a + b;
				a = placeholder;
				result = b
			}
			return result as u64;
		}
	}
}


fn main() {
    println!("Hello, world!");
    let result = fibonacci_number_none_recur(20);
    println!("{}", result);
}
