use crate::code_generator::expression_parser::expression::Expression;
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
        self.add()
    }

    fn add(&mut self) -> Expression {
        let mut node = self.mul();

        loop {
            if self.expr.consume_reserved_token(ReservedToken::Add).unwrap() {
                node = Expression::Add(Box::new(node), Box::new(self.mul()));
            } else if self.expr.consume_reserved_token(ReservedToken::Sub).unwrap() {
                node = Expression::Sub(Box::new(node), Box::new(self.mul()));
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Expression {
        let mut node = self.primary();

        loop {
            if self.expr.consume_reserved_token(ReservedToken::Mul).unwrap() {
                node = Expression::Mul(Box::new(node), Box::new(self.primary()));
            } else if self.expr.consume_reserved_token(ReservedToken::Div).unwrap() {
                node = Expression::Div(Box::new(node), Box::new(self.primary()));
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
