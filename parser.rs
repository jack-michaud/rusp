use std::fs::File;

use expressions;

macro_rules! make_args {

    ($($x:expr),*) => {
    	{
    		let mut toReturn = Vec::new();
	    	$(
	    		toReturn.push(
	    			expressions::Argument::Atom {
	    				content: Some(String::from($x))
	    			}
	    		);
	    	)*
    		toReturn
    	}
    }
}

pub fn parse_file(f: File) -> expressions::Argument {
	let args = make_args!("2", "3");
    let myExp = expressions::Argument::new(String::from("+"), args);
	expressions::Argument::new(String::from("+"), Vec::new())
}