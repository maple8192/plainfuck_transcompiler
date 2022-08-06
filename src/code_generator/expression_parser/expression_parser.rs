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
        let lhs = self.mul();

        if self.expr.consume_reserved_token(ReservedToken::Add).unwrap() {
            return Expression::Add(Box::new(lhs), Box::new(self.add()));
        } else if self.expr.consume_reserved_token(ReservedToken::Sub).unwrap() {
            return Expression::Sub(Box::new(lhs), Box::new(self.add()));
        }

        lhs
    }

    fn mul(&mut self) -> Expression {
        let lhs = self.primary();

        if self.expr.consume_reserved_token(ReservedToken::Mul).unwrap() {
            return Expression::Mul(Box::new(lhs), Box::new(self.mul()));
        } else if self.expr.consume_reserved_token(ReservedToken::Div).unwrap() {
            return Expression::Div(Box::new(lhs), Box::new(self.mul()));
        }

        lhs
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
