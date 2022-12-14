use crate::code_generator::code_exporter::export_code;
use crate::code_generator::code_generator::generate_code;
use crate::tokenizer::token_type::TokenType;
use crate::tokenizer::tokenizer::tokenize;

mod arguments_handler;
mod code_reader;
mod tokenizer;
mod code_generator;

fn main() {
    let code_path = arguments_handler::get_code_path().unwrap();
    let code = code_reader::read_content(code_path).unwrap();

    let token_queue = tokenize(code).unwrap();

    let result = generate_code(token_queue);
    export_code(result, "result.bf");
}
