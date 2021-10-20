use std::io;

fn fact_loop(n: i32) -> i32 {
	if n == 0 {
		1
	} else {
		let mut result: i32 = 1;
		for i in 1..(n+1) {
			result *= i;
		}
		result
	}
}
fn fact_recursive(n: i32) -> i32 {
	if n == 0 {
		1
	} else {
		n * fact_recursive(n - 1)
	}
}

fn main() {
	println!("Enter a number you want to calculate factorial of: ");
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let n = input.trim().parse().unwrap();

	println!("Factorial (calculated using loop): {}! = {}", n, fact_loop(n));
	println!("Factorial (calculated using recursion): {}! = {}", n, fact_recursive(n));
}