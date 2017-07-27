use std::env;

use std::fs::File;

mod parser;
mod expressions;
mod eval;

fn main() {
    
    let cl_args = env::args();
    if cl_args.count() < 2 {
        println!("Must provide a rusp file. Usage: ./cargo run <example.lisp>");
        return;
    }


    let file = match File::open("test.lisp") {
        Ok(v) => v,
        Err(_) => panic!("Couldnt open rusp file.")
    };
    let my_exp = parser::parse_file(file);
    
    println!("{:?}", my_exp.stringify());
    println!("{}", eval::evaluate(&my_exp));
}
