use crate::tokenizer::reserved_token::ReservedToken;

pub enum TokenType {
    Reserved(ReservedToken),
    Number(u32),
    String(String),
    End,
}
