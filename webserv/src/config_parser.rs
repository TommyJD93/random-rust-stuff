mod parser;

pub fn parser(path: String) {
	let config = parser::read_file(path);
	
	println!("[WebServ]:");
	println!("{}", config);

}
