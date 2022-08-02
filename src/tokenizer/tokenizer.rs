use std::any::Any;
use std::collections::VecDeque;
use crate::tokenizer::reserved_token::ReservedToken;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

pub fn tokenize(code: String) -> Result<VecDeque<Token>, String> {
    let mut tokens = VecDeque::<Token>::new();

    let mut p = 0;
    while p < code.len() {
        match code.chars().nth(p).unwrap() {
            ' ' | '\n' | '\r' | '\t' => (),
            '+' => tokens.push_back(Token { token_type: TokenType::Reserved(ReservedToken::Add) }),
            '-' => tokens.push_back(Token { token_type: TokenType::Reserved(ReservedToken::Sub) }),
            '0'..='9' => {
                let mut s = String::from(code.chars().nth(p).unwrap());
                for i in (p + 1)..code.len() {
                    if let '0'..='9' = code.chars().nth(i).unwrap() {
                        s.push(code.chars().nth(i).unwrap());
                    } else {
                        p = i;
                        break;
                    }
                }
                tokens.push_back(Token { token_type: TokenType::Number(s.parse::<u32>().unwrap()) });
            }
            _ => return Err("".to_string()),
        }
        p += 1;
    }

    Ok(tokens)
}
