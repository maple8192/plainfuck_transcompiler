use crate::tokenizer::reserved_token::ReservedToken;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_queue::TokenQueue;
use crate::tokenizer::token_type::TokenType;

pub fn tokenize(code: String) -> Result<TokenQueue, String> {
    let mut queue = TokenQueue::new();

    let mut p = 0;
    while p < code.len() {
        match code.chars().nth(p).unwrap() {
            ' ' | '\n' | '\r' | '\t' => (),
            '+' => queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Add) }),
            '-' => queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Sub) }),
            '*' => queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Mul) }),
            '/' => queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Div) }),
            '(' => queue.add(Token { token_type: TokenType::Reserved(ReservedToken::OpenBracket) }),
            ')' => queue.add(Token { token_type: TokenType::Reserved(ReservedToken::CloseBracket) }),
            '=' => if code.chars().nth(p + 1).unwrap() == '=' { queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Equal) }); p += 1; } else { queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Assign) }); }
            '!' => if code.chars().nth(p + 1).unwrap() == '=' { queue.add(Token { token_type: TokenType::Reserved(ReservedToken::NotEqual) }); p += 1; } else { return Err("".to_string()); }
            '<' => if code.chars().nth(p + 1).unwrap() == '=' { queue.add(Token { token_type: TokenType::Reserved(ReservedToken::LessOrEqual) }); p += 1; } else { queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Less) }); }
            '>' => if code.chars().nth(p + 1).unwrap() == '=' { queue.add(Token { token_type: TokenType::Reserved(ReservedToken::GreaterOrEqual) }); p += 1; } else { queue.add(Token { token_type: TokenType::Reserved(ReservedToken::Greater) }); }
            ';' => queue.add(Token { token_type: TokenType::Reserved(ReservedToken::EndStatement) }),
            '0'..='9' => {
                let mut s = String::from(code.chars().nth(p).unwrap());
                for i in (p + 1)..code.len() {
                    if let '0'..='9' = code.chars().nth(i).unwrap() {
                        s.push(code.chars().nth(i).unwrap());
                    } else {
                        p = i - 1;
                        break;
                    }
                }
                queue.add(Token { token_type: TokenType::Number(s.parse::<u32>().unwrap()) });
            }
            'a'..='z' | 'A'..='Z' => {
                let mut s = String::from(code.chars().nth(p).unwrap());
                for i in (p + 1)..code.len() {
                    if let 'a'..='z' | 'A'..='Z' = code.chars().nth(i).unwrap() {
                        s.push(code.chars().nth(i).unwrap());
                    } else {
                        p = i - 1;
                        break;
                    }
                }
                queue.add(Token { token_type: TokenType::Ident(s) });
            }
            _ => return Err("".to_string()),
        }
        p += 1;
    }

    queue.add(Token { token_type: TokenType::End });

    Ok(queue)
}
