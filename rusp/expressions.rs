use std::fmt;


pub enum Argument {
    Expression {
        func:   String,
        args: Box<Vec<Argument>>,
    },
    Atom {
        content: Option<i32>
    }
}


impl Argument {

    pub fn new(func: String, args: Vec<Argument>) -> Argument {
        Argument::Expression { func: func, args: Box::new(args) }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Argument::Expression { ref args, ref func } => {
                println!("We got an expression");
                let mut to_return = String::from("(");
                to_return.push_str(func.as_str());
                to_return.push_str(" ");
                for arg in args.iter() {
                    to_return.push_str(&arg.stringify().as_str());
                    to_return.push_str(" ");
                }
                to_return.push_str(")");
                to_return
            },
            Argument::Atom { ref content } => {
                println!("We got an Atom");
                match *content {
                    Some(ref i) => {
                        let mut to_return = String::from("");
                        to_return.push_str(i.to_string().as_str());
                        to_return
                    },
                    None => String::from("")
                }
                
            }

        }
    }
}

