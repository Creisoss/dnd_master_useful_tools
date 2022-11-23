use std::io;
pub fn input_char() -> char{
	let mut input_text = String::new();
	io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
	let trimmed = input_text.trim();
	match trimmed.parse::<char>() {
		Ok(i) =>   i,
		Err(..) => '\0',
	}
}

pub fn input_number() -> u32{
	let mut input_text = String::new();
	io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
	let trimmed = input_text.trim();
	match trimmed.parse::<u32>() {
    	Ok(i) =>   i,
	    Err(..) => 0,
    }	
}


