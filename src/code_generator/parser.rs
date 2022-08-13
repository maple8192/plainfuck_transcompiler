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
            expr_queue.add(self.statement());
        }

        expr_queue
    }

    fn statement(&mut self) -> Node {
        return if self.program.consume_reserved_token(ReservedToken::If).unwrap() {
            if !self.program.consume_reserved_token(ReservedToken::OpenBracket).unwrap() {
                panic!("");
            }

            let condition = self.equality();

            if !self.program.consume_reserved_token(ReservedToken::CloseBracket).unwrap() {
                panic!("")
            }

            let statement0 = self.statement();

            let statement1 = if self.program.consume_reserved_token(ReservedToken::Else).unwrap() {
                Some(Box::new(self.statement()))
            } else {
                None
            };

            Node::If(Box::new(condition), Box::new(statement0), statement1)
        } else {
            let node = self.assign();

            if !self.program.consume_reserved_token(ReservedToken::EndStatement).unwrap() {
                panic!("");
            }

            node
        }
    }

    fn assign(&mut self) -> Node {
        let node = self.equality();

        if self.program.consume_reserved_token(ReservedToken::Assign).unwrap() {
            Node::BinaryOperator(BinaryOperatorType::Assign, Box::new(node), Box::new(self.equality()))
        } else {
            node
        }
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
        } else if let Ok(name) = self.program.consume_ident_token() {
            return Node::Variable(name);
        }

        Node::Number(self.program.consume_number_token().unwrap())
    }
}
