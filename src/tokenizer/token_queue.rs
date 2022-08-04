use std::collections::vec_deque::VecDeque;
use crate::tokenizer::reserved_token::ReservedToken;
use crate::tokenizer::token::Token;
use crate::TokenType;

pub struct TokenQueue {
    tokens: VecDeque<Token>,
}

impl TokenQueue {
    pub fn new() -> Self {
        TokenQueue { tokens: VecDeque::new() }
    }

    pub fn add(&mut self, token: Token) {
        self.tokens.push_back(token);
    }

    pub fn consume_reserved_token(&mut self, request: ReservedToken) ->Result<(), ()> {
        let front = self.tokens.front();

        if let Some(token) = front {
            if let TokenType::Reserved(t) = &token.token_type {
                if t == &request {
                    self.tokens.pop_front();
                    return Ok(());
                }
            }

            Err(())
        } else {
            Err(())
        }
    }

    pub fn consume_number_token(&mut self) -> Result<u32, ()> {
        let front = self.tokens.front();

        if let Some(token) = front {
            if let TokenType::Number(n) = token.token_type {
                self.tokens.pop_front();
                return Ok(n);
            }
        }

        Err(())
    }

    pub fn is_end(&self) -> bool {
        self.tokens.front().unwrap().token_type == TokenType::End
    }
}
