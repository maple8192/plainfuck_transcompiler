use crate::tokenizer::reserved_token::ReservedToken;

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Reserved(ReservedToken),
    Number(u32),
    End,
}
