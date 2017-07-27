
use std::collections::HashMap;
use expressions::Argument;

pub fn evaluate(expr: &Argument) -> i32 {
	match expr {
		&Argument::Expression(ref func, ref args) => {
			if func == "+" {
				args.iter().map(evaluate).sum()
			} else if func == "-" {
				if args.len() == 1 {
					return -1 * evaluate(args.first().unwrap())
				}
				let mut evaluated_args: Vec<i32> = args.iter().map(evaluate).collect();
				let mut result = evaluated_args.first().unwrap().clone();
				evaluated_args.remove(0);
				for arg in evaluated_args {
					result = result - arg;
				}
				result
			} else if func == "*" {
				args.iter().map(evaluate).fold(1, |acc, arg| {
					return acc * arg
				})
			} else {
				1
			}
		},
		&Argument::Atom(val) => {
			val.unwrap()
		}
	}
}


