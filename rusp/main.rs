use std::env;
use std::collections::HashMap;

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

    let mut handler = eval::EventHandler { functions: HashMap::new() };
    handler.add_function("+".to_string(), eval::add);
    handler.add_function("-".to_string(), eval::sub);
    handler.add_function("*".to_string(), eval::mult);
    handler.add_function("/".to_string(), eval::division);
    handler.add_function("double".to_string(), eval::double_all);
    handler.add_function("pow".to_string(), eval::power_of);

    println!("{}", eval::evaluate(&my_exp, &mut handler));
}
