use crate::tokenizer::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    next: Box<Option<Token>>,
}
