use std::fs::File;
use std::io::prelude::*;

use expressions;

macro_rules! make_args {

    ($($x:expr),*) => {
    	{
    		let mut to_return = Vec::new();
	    	$(
	    		to_return.push(
	    			expressions::Argument::Atom {
	    				content: Some($x.parse::<i32>().unwrap())
	    			}
	    		);
	    	)*
    		to_return
    	}
    }
}

pub fn parse_file(mut f: File) -> expressions::Argument {
	let args = make_args!("2", "3");
	let mut file_string = String::new();
	f.read_to_string(&mut file_string); 
	println!("{:?}", tokenize(file_string));
    let my_exp = expressions::Argument::new(String::from("+"), args);
	// expressions::Argument::new(String::from("+"), Vec::new())
	my_exp
}

pub fn tokenize(mut file: String) -> Vec<String> {
	let length = file.len();

	// let ref mut temp_string = String::new();
	let mut string_flag = false;

	let arr: Vec<String> = file.drain(..length).fold(Vec::new(), |mut acc, s| {
		if s == '"' {
			if !string_flag {
				string_flag = true;
			} else {
				// acc.push(temp_string);
				// temp_string = String::new();
				string_flag = false;
			}
			acc.push(s.to_string());
			acc
		} else if s == ' ' {
			if string_flag {
				acc.push(s.to_string());
				acc
			} else {
				acc
			}
		} else {
			acc.push(s.to_string());
			acc
		}
		
	});

	arr
}
// fn parse(tokens: Vec<String>) -> expressions::Argument {
// 	// String parse
// 	// let four: u32 = "4".parse().unwrap();
// 	expressions
// }
