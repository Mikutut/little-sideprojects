use std::io::{self, Write};

type FactType = i128;

fn fact_loop(n: FactType) -> FactType {
	if n == 0 {
		1
	} else {
		let mut result: FactType = 1;
		for i in 1..(n+1) {
			result *= i;
		}
		result
	}
}
fn fact_recursive(n: FactType) -> FactType {
	if n == 0 {
		1
	} else {
		n * fact_recursive(n - 1)
	}
}

fn main() {
	loop {
		print!("Enter a number you want to calculate factorial of: ");
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let n = input.trim().parse::<FactType>().unwrap();

		println!("Factorial (calculated using loop): {}! = {}", n, fact_loop(n));
		println!("Factorial (calculated using recursion): {}! = {}", n, fact_recursive(n));
	}
}