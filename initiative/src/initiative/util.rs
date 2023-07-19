/***************************************************************************************************
* Initiative Tracker
* Version 0.4.0 (07/14/23)
* input.rs
* 0xMattB
***************************************************************************************************/

use std::io::{self, Write, stdout};

pub fn get_string() -> String {
	let mut input = String::new();
	
	if read(&mut input) {
		input = input.trim().to_lowercase();
	}
	
	input
}

pub fn get_i32(num: &mut i32) -> bool {
	let input = get_string();

	match input.parse::<i32>() {
		Ok(n) => {
			*num = n;
			true
		},
		Err(..) => {
			false
		}
	}
}

pub fn get_i32_from_string(s: &str, num: &mut i32) -> bool {
	match s.parse::<i32>() {
		Ok(n) => {
			*num = n;
			true
		},
		Err(..) => {
			false
		}
	}
}

pub fn print_without_newline(s: &str) {
	print!("{}", s);
	let _ = stdout().flush();
}
	
fn read(dest: &mut String) -> bool {
	match io::stdin().read_line(dest) {
		Ok(_) => true,
		Err(_) => false,
	}
}