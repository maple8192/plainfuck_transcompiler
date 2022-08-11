use crate::code_generator::expression_parser::expression::{BinaryOperatorType, Expression};
use crate::tokenizer::reserved_token::ReservedToken;
use crate::tokenizer::token_queue::TokenQueue;

pub struct ExpressionParser {
    expr: TokenQueue,
}

impl ExpressionParser {
    pub fn new(expr: TokenQueue) -> Self {
        ExpressionParser { expr }
    }

    pub fn parse(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut node = self.relational();

        loop {
            if self.expr.consume_reserved_token(ReservedToken::Equal).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::Equal, Box::new(node), Box::new(self.relational()));
            } else if self.expr.consume_reserved_token(ReservedToken::NotEqual).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::NotEqual, Box::new(node), Box::new(self.relational()));
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Expression {
        let mut node = self.add();

        loop {
            if self.expr.consume_reserved_token(ReservedToken::Less).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::Less, Box::new(node), Box::new(self.add()));
            } else if self.expr.consume_reserved_token(ReservedToken::LessOrEqual).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::LessOrEqual, Box::new(node), Box::new(self.add()));
            } else if self.expr.consume_reserved_token(ReservedToken::Greater).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::Greater, Box::new(node), Box::new(self.add()));
            } else if self.expr.consume_reserved_token(ReservedToken::GreaterOrEqual).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::GreaterOrEqual, Box::new(node), Box::new(self.add()));
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Expression {
        let mut node = self.mul();

        loop {
            if self.expr.consume_reserved_token(ReservedToken::Add).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::Add, Box::new(node), Box::new(self.mul()));
            } else if self.expr.consume_reserved_token(ReservedToken::Sub).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::Sub, Box::new(node), Box::new(self.mul()));
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Expression {
        let mut node = self.primary();

        loop {
            if self.expr.consume_reserved_token(ReservedToken::Mul).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::Mul, Box::new(node), Box::new(self.primary()));
            } else if self.expr.consume_reserved_token(ReservedToken::Div).unwrap() {
                node = Expression::BinaryOperator(BinaryOperatorType::Div, Box::new(node), Box::new(self.primary()));
            } else {
                return node;
            }
        }
    }

    fn primary(&mut self) -> Expression {
        if self.expr.consume_reserved_token(ReservedToken::OpenBracket).unwrap() {
            let node = self.add();
            if !self.expr.consume_reserved_token(ReservedToken::CloseBracket).unwrap() { panic!(""); }
            return node;
        }

        Expression::Number(self.expr.consume_number_token().unwrap())
    }
}
