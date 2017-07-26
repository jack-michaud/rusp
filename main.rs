use std::env;

use std::fs::File;

mod parser;
mod expressions;
// const foo: str = include_str!("test.lisp");

fn main() {
	
	let clArgs = env::args();
	if clArgs.count() < 2 {
		println!("Must provide a rusp file. Usage: ./main <example.lisp>");
		return;
	}

	let file = match File::open("test.lisp") {
		Ok(v) => v,
		Err(e) => panic!("Couldnt open rusp file.")
	};
	let myExp = parser::parse_file(file);
	
    println!("{:?}", myExp.stringify());
}
