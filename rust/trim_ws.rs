use std::fs::File;
use std::io::*;
use std::env;

fn write_to_file(file: &mut File, output_data: &String) -> () {
	match file.write(output_data.as_bytes()) {
		Ok(_) => (),
		Err(err) => {
			println!("File write error! {}", err.to_string());
		}
	}	
}

fn main() {
	let raw_args: Vec<String> = env::args().collect();
	let in_path: &String = &raw_args[1];
	let op_path: &String = &raw_args[2];

	let mut read_data = String::new();
	
	let file: Result<File> = File::open(in_path);

	match file {
		Ok(mut f) => {
			match f.read_to_string(&mut read_data) {
				Ok(_) => {
					let read_data: Vec<&str> = read_data.trim().split_whitespace().collect();
					let output_data: String = read_data.join("");

					match File::open(op_path) {
						Ok(mut f) => {
							write_to_file(&mut f, &output_data);
						},
						Err(_) => {
							match File::create(op_path) {
								Ok(mut f) => {
									write_to_file(&mut f, &output_data);
								},
								Err(err) => {
									println!("File creation error! {}", err.to_string());
								}
							}
						}
					}
				},
				Err(err) => {
					println!("File read error! {}", err.to_string());
				}
			}
		},
		Err(err) => {
			println!("File read error! {}", err.to_string());
		}
	}
}