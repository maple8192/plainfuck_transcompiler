use crate::code_generator::code_data::CodeData;
use crate::code_generator::command::Command;
use crate::tokenizer::reserved_token::ReservedToken;
use crate::tokenizer::token_queue::TokenQueue;

pub fn generate_code(mut queue: TokenQueue) -> Result<CodeData, ()> {
    let mut code_data = CodeData::new();

    code_data.add_command(Command::Add(queue.consume_number_token().unwrap()));

    while !queue.is_end() {
        if queue.consume_reserved_token(ReservedToken::Add).is_ok() {
            code_data.add_command(Command::Add(queue.consume_number_token().unwrap()));
        } else if queue.consume_reserved_token(ReservedToken::Sub).is_ok() {
            code_data.add_command(Command::Sub(queue.consume_number_token().unwrap()));
        } else {
            panic!("");
        }
    }

    Ok(code_data)
}
