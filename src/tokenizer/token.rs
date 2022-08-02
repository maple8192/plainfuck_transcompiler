use crate::tokenizer::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub next: Box<Option<Token>>,
}
