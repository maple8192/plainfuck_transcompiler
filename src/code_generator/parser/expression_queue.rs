use std::collections::vec_deque::VecDeque;
use crate::code_generator::parser::node::Node;

pub struct ExpressionQueue {
    queue: VecDeque<Node>,
}

impl ExpressionQueue {
    pub fn new() -> Self {
        ExpressionQueue { queue: VecDeque::new() }
    }

    pub fn add(&mut self, expr: Node) {
        self.queue.push_back(expr);
    }

    pub fn consume(&mut self) -> Option<Node> {
        self.queue.pop_front()
    }
}
