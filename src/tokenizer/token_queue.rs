use std::collections::vec_deque::VecDeque;
use crate::tokenizer::token::Token;

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
}
