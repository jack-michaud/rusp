use std::fmt;


pub enum Argument {
    Expression(String, Box<Vec<Argument>>),
    Atom(Option<i32>)
}

impl Argument {

    pub fn new(func: String) -> Argument {
        Argument::Expression(func, Box::new(Vec::new()))
    }

    pub fn arg_length(self) -> usize {
        match self {
            Argument::Expression (_, args) => args.len(),
            Argument::Atom(_) => 0
        }

    }

    pub fn add_arg(&mut self, a: Argument) {
        match *self {
            Argument::Expression(ref func, ref mut args) => {
                args.push(a);
            },
            _ => panic!("Cannot push argument to Atom"),
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Argument::Expression(ref func, ref args) => {
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
            Argument::Atom(ref content) => {
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

