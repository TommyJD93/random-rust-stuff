use std::fs;

pub enum ConfigError {
	InvalidLine(usize, String),
	EmptyFile,
}

pub fn first_file_validation(config: String) -> Result<usize, ConfigError> {
	let lines: Vec<&str> = config.lines().collect();

	if lines.is_empty() {
		return Err(ConfigError::EmptyFile);
	}

	

	Ok(lines.len())
}

pub fn read_file(path: String) -> String {
	let config = fs::read_to_string(path)
		.expect("[WebServ]: Unable to open file");

	match first_file_validation(config.clone()) {
		Ok(count) => {
			println!("[WebServ]: number of lines: {}", count);
			return config;
		}
		Err(ConfigError::InvalidLine(line, content)) => {
			println!("[WebServ]:Error on line {}: {}", line, content);
			return String::from("");
		}		
		Err(ConfigError::EmptyFile) => {
			println!("[WebServ]: Config File is empty");
			return String::from("");
		}
	}
}
