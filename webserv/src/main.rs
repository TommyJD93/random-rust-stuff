use std::env;
use nom;
mod config_parser;

fn main() {
	let args: Vec<String> =  env::args().collect();
	println!("{}", &args[1]);

	config_parser::parser(args[1].clone());

}
