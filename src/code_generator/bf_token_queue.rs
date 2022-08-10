use std::collections::vec_deque::VecDeque;
use crate::code_generator::bf_token::BFToken;

pub struct BFTokenQueue {
    queue: VecDeque<BFToken>
}

impl BFTokenQueue {
    pub fn new() -> Self {
        BFTokenQueue { queue: VecDeque::new() }
    }

    pub fn add_token(&mut self, token: BFToken) {
        self.queue.push_back(token);
    }

    pub fn consume_token(&mut self) -> Option<BFToken> {
        self.queue.pop_front()
    }
}
