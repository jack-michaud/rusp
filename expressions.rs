use std::fmt;


pub enum Argument {
    Expression {
	    func:   String,
	    args: Box<Vec<Argument>>,
	},
    Atom(Atom)
}

pub struct Atom {
    content: Option<Value>
}

pub enum Value {
    String,
    i32,
}


impl Argument {

    pub fn new(func: String, args: Vec<Argument>) -> Argument {
        Argument::Expression { func: func, args: Box::new(args) }
    }

    pub fn stringify(&self) -> String {
    	match *self {
    		Argument::Expression { args: ref args, func: ref func } => {
    			println!("We got an expression");
    			let mut toReturn = String::from("(");
                toReturn.push_str(func.as_str());
                toReturn.push_str(" ");
                for arg in args.iter() {
                    toReturn.push_str(&arg.stringify().as_str());
                    toReturn.push_str(" ");
                }
                toReturn.push_str(")");
                toReturn
    		},
    		Argument::Atom(content) => {
    			println!("We got an Atom");
                match content.content {
                    Some(i) => {
                        let mut toReturn = String::from("");
                        toReturn.push_str(content.as_str());
                        toReturn
                    },
                    None => ()
                }
                
    		}

    	}
    }
  //   	match *self {
		// 	Argument::Expression { ref func, ref args } => {
		// 		let &result = format!("({} {})", func, args.iter().fold(String::from(""), |mut acc, arg| {
		// 			let child_arg = arg.stringify();
		// 			acc.push_str(child_arg.as_str());
		// 			acc
		// 		}));
		// 		result
		//	},
		// 	Argument::Atom {ref content, .. } => content,
		// }
 //   }
}

