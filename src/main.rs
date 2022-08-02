use crate::tokenizer::token_type::TokenType;
use crate::tokenizer::tokenizer::tokenize;

mod arguments_handler;
mod code_reader;
mod tokenizer;

fn main() {
    let code_path = arguments_handler::get_code_path().unwrap();
    let code = code_reader::read_content(code_path).unwrap();

    let mut tokens = tokenize(code).unwrap();

    for _ in 0..tokens.len() {
        let token = tokens.pop_front();

        if let Some(t) = token {
            match t.token_type {
                TokenType::Reserved(_) => println!("Reserved"),
                TokenType::Number(n) => println!("Number: {}", n),
            }
        } else {
            println!("Error");
        }
    }
}
