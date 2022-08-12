use crate::code_generator::parser::expression_queue::ExpressionQueue;
use crate::code_generator::parser::node::binary_operator_type::BinaryOperatorType;
use crate::code_generator::parser::node::Node;
use crate::tokenizer::reserved_token::ReservedToken;
use crate::tokenizer::token_queue::TokenQueue;

pub mod node;
pub mod expression_queue;

pub struct Parser {
    program: TokenQueue,
}

impl Parser {
    pub fn new(tokens: TokenQueue) -> Self {
        Parser { program: tokens }
    }

    pub fn parse(&mut self) -> ExpressionQueue {
        let mut expr_queue = ExpressionQueue::new();

        while !self.program.is_end() {
            expr_queue.add(self.equality());

            if !self.program.consume_reserved_token(ReservedToken::EndStatement).unwrap() {
                panic!("");
            }
        }

        expr_queue
    }

    fn equality(&mut self) -> Node {
        let mut node = self.relational();

        loop {
            if self.program.consume_reserved_token(ReservedToken::Equal).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::Equal, Box::new(node), Box::new(self.relational()));
            } else if self.program.consume_reserved_token(ReservedToken::NotEqual).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::NotEqual, Box::new(node), Box::new(self.relational()));
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Node {
        let mut node = self.add();

        loop {
            if self.program.consume_reserved_token(ReservedToken::Less).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::Less, Box::new(node), Box::new(self.add()));
            } else if self.program.consume_reserved_token(ReservedToken::LessOrEqual).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::LessOrEqual, Box::new(node), Box::new(self.add()));
            } else if self.program.consume_reserved_token(ReservedToken::Greater).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::Greater, Box::new(node), Box::new(self.add()));
            } else if self.program.consume_reserved_token(ReservedToken::GreaterOrEqual).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::GreaterOrEqual, Box::new(node), Box::new(self.add()));
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Node {
        let mut node = self.mul();

        loop {
            if self.program.consume_reserved_token(ReservedToken::Add).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::Add, Box::new(node), Box::new(self.mul()));
            } else if self.program.consume_reserved_token(ReservedToken::Sub).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::Sub, Box::new(node), Box::new(self.mul()));
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Node {
        let mut node = self.primary();

        loop {
            if self.program.consume_reserved_token(ReservedToken::Mul).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::Mul, Box::new(node), Box::new(self.primary()));
            } else if self.program.consume_reserved_token(ReservedToken::Div).unwrap() {
                node = Node::BinaryOperator(BinaryOperatorType::Div, Box::new(node), Box::new(self.primary()));
            } else {
                return node;
            }
        }
    }

    fn primary(&mut self) -> Node {
        if self.program.consume_reserved_token(ReservedToken::OpenBracket).unwrap() {
            let node = self.add();
            if !self.program.consume_reserved_token(ReservedToken::CloseBracket).unwrap() { panic!(""); }
            return node;
        }

        Node::Number(self.program.consume_number_token().unwrap())
    }
}
