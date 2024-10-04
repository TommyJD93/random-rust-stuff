pub enum TokenType {
    TokAlfa, TokNumber, TokString, TokSemiColumn,

    TokOpenBracket, TokCloseBracket,

    TokError,
}


fn assign_token_type() -> TokenType {
}

fn check_for_errors() -> bool {
}

fn move_tok_to_vec() {
}

pub fn tokenizer(conf_lines: Vec<String>) {
    for line in conf_lines {
        let token_vec: Vec<&str> = line.split_whitespace().collect();

        assign_token_type();

        if check_for_errors() {

        }

        move_tok_to_vec();
    }
}