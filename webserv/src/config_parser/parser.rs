use std::fs;

pub enum ConfigError {
	InvalidLine(usize, String),
	EmptyFile,
}

pub fn first_file_validation(config: String) -> Result<Vec<String>, ConfigError> {
	let lines: Vec<&str> = config.lines().collect();

	if lines.is_empty() {
		return Err(ConfigError::EmptyFile);
	}

	let mut line_num = 0;

	for line in lines {
		line_num += 1;
		if let Some(last_char) = line.chars().last() {
			if last_char != '}' && last_char != ';' && last_char != '{' {
				return Err(ConfigError::InvalidLine(line_num, line.trim().to_string()));
			}
		}
	}

	// this requires a better way to convert the &str vec to a String vec
	let lines: Vec<&str> = config.lines().collect();
	let lines: Vec<String> = lines.into_iter().map(|line| line.to_string()).collect();

	Ok(lines)
}

pub fn read_file(path: String) -> String {
	let config = fs::read_to_string(path)
		.expect("[WebServ]: Unable to open file");

	match first_file_validation(config.clone()) {
		Ok(conf_lines) => {
			println!("[WebServ]: number of lines: {}", conf_lines.len());
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
