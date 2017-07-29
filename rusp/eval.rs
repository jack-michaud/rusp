
use std::collections::HashMap;
use expressions::Argument;


pub fn evaluate(expr: &Argument, handler: &mut EventHandler) -> i32 {
	match expr {
		&Argument::Expression(ref func, ref args) => {
			handler.execute(func.clone(), *args.clone()).clone()
		},
		&Argument::Atom(val) => {
			val.unwrap()
		}
	}
}

// Arguments, the handler context
pub type Function = fn(Vec<Argument>, &mut EventHandler) -> i32;

pub struct EventHandler {
	pub functions: HashMap<String, Function>
}

impl EventHandler {

	pub fn add_function(&mut self, name: String, func: Function) {
		self.functions.insert(name, func);
	}

	pub fn execute(&mut self, name: String, args: Vec<Argument>) -> i32 {
		let function = *self.functions.get(&name).unwrap();
		function(args, self)

	}
}

pub fn add(args: Vec<Argument>, handler: &mut EventHandler) -> i32 {
    let evaluate_with_handler = |a: &Argument| -> i32 { evaluate(a, handler) };
    args.iter().map(evaluate_with_handler).sum()
}

pub fn sub(args: Vec<Argument>, handler: &mut EventHandler) -> i32 {
	let mut evaluate_with_handler = |a: &Argument| -> i32 { evaluate(a, handler) };
	if args.len() == 1 {
		return -1 * evaluate_with_handler(&args.first().unwrap().clone())
	}
	let mut evaluated_args: Vec<i32> = args.iter().map(evaluate_with_handler).collect();
	let mut result = evaluated_args.first().unwrap().clone();
	evaluated_args.remove(0);
	for arg in evaluated_args {
		result = result - arg;
	}
	result
}

pub fn mult(args: Vec<Argument>, handler: &mut EventHandler) -> i32 {
	let evaluate_with_handler = |a: &Argument| -> i32 { evaluate(a, handler) };
	args.iter().map(evaluate_with_handler).fold(1, |acc, arg| {
		return acc * arg
	})
}

pub fn division(args: Vec<Argument>, handler: &mut EventHandler) -> i32 {
	let evaluate_with_handler = |a: &Argument| -> i32 { evaluate(a, handler) };
	let divide = |acc: i32, a: &i32| -> i32 { 
		println!("{} / {}", acc, a);
		acc / a 
	};

	let mut evaluated_args: Vec<i32> = args.iter().map(evaluate_with_handler).collect();
	let first = evaluated_args.first().unwrap().clone();
	evaluated_args.remove(0);

	evaluated_args.iter().fold(first, divide)
}

pub fn double_all(args: Vec<Argument>, handler: &mut EventHandler) -> i32 {
	let mut evaluate_with_handler = |a: &Argument| -> i32 { evaluate(a, handler) };
	let double = |a: &Argument| -> i32 { 2 * evaluate_with_handler(a) };
	args.iter().map(double).sum()
}

pub fn power_of(args: Vec<Argument>, handler: &mut EventHandler) -> i32 {
	let evaluate_with_handler = |a: &Argument| -> i32 { evaluate(a, handler) };
	let power = |acc: i32, a: &i32| -> i32 { acc.pow(*a as u32) };

	let mut evaluated_args: Vec<i32> = args.iter().map(evaluate_with_handler).collect();
	let first = evaluated_args.first().unwrap().clone();
	evaluated_args.remove(0);

	evaluated_args.iter().fold(first, power)
}

// pub fn template(args: Vec<Argument>, handler: &mut EventHandler) -> i32 {
// 	...
// }
