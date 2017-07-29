use std::fs::File;
use std::io::prelude::*;

use expressions;

macro_rules! make_args {

    ($($x:expr),*) => {
        {
            let mut to_return = Vec::new();
            $(
                to_return.push(
                    expressions::Argument::Atom(Some($x.parse::<i32>().unwrap()))
                );
            )*
            to_return
        }
    }
}

pub fn parse_file(mut f: File) -> expressions::Argument {
    let mut file_string = String::new();
    let _ = f.read_to_string(&mut file_string); 
    let mut tokenized_expr = tokenize(file_string);
    tokenized_expr.remove(0);
    let (my_exp, _) = parse(tokenized_expr);
    // expressions::Argument::new(String::from("+"), Vec::new())
    my_exp
}

pub fn tokenize(mut file: String) -> Vec<String> {
    let length = file.len();

    // let ref mut temp_string = String::new();


    let mut drainer = file.drain(..length);
    let mut arr: Vec<String> = Vec::new();
    let mut started_value = false;
    let mut accumulated_value = String::new();

    while {
        let (start, end) = drainer.size_hint();
        start
    } > 0 {
        let s = drainer.next().unwrap();

        if s == '(' || s == ')' {
            if started_value {
                started_value = false;
                arr.push(accumulated_value);
                accumulated_value = String::new();
            }
            arr.push(s.to_string());
        } else if s == '"' {
            arr.push(s.to_string());
            let mut temp_string = String::new();

            loop {
                let temp = drainer.next().unwrap();
                if temp == '\"' {
                    arr.push(temp_string);
                    break;
                } else {
                    temp_string.push(temp);
                }
            }
        } else if s == ' ' {
            if started_value {
                started_value = false;
                arr.push(accumulated_value);
                accumulated_value = String::new();
            }
        } else {
            started_value = true;
            accumulated_value.push(s);
        }
    }
    arr
}

fn parse(mut tokens: Vec<String>) -> (expressions::Argument,i32) {
    let func = tokens.first().unwrap().clone();
    tokens.remove(0);

    let mut arg = expressions::Argument::new(func);

    let mut count = 1;
    while tokens.len() > 0 {
        let next = tokens.first().unwrap().clone();
        tokens.remove(0);
        count += 1;
        if next == ")" {
            break
        }
        if next == "(" {

            let (new_arg, jump_amount) = parse(tokens.clone());
            arg.add_arg(new_arg);
            for _ in 0..jump_amount {
                tokens.remove(0);
            }
        } else {
            match next.parse::<i32>() {
                Ok(r) => {
                    arg.add_arg(expressions::Argument::Atom(Some(r)));
                },
                Err(_) => {},
            }
        }
    }

    (arg, count)
}

