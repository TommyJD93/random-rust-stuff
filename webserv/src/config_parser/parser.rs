use std::fs;

pub fn read_file(path: String) -> String{
	return fs::read_to_string(path)
		.expect("unable to open file");
}
