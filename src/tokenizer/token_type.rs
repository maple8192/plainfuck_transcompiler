use crate::tokenizer::reserved_token::ReservedToken;

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Reserved(ReservedToken),
    Ident(String),
    Number(u32),
    End,
}
