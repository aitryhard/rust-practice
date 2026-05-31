use crate::ast::{BinOpKind, Expr};
use crate::error::CalcError;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

/// Binary operator precedence (lower = binds looser).
fn precedence(op: &BinOpKind) -> u8 {
    match op {
        BinOpKind::Add | BinOpKind::Sub => 1,
        BinOpKind::Mul | BinOpKind::Div | BinOpKind::Modulo => 2,
        BinOpKind::Pow => 3,
    }
}

fn token_to_binop(token: &Token) -> Option<BinOpKind> {
    match token {
        Token::Plus => Some(BinOpKind::Add),
        Token::Minus => Some(BinOpKind::Sub),
        Token::Star => Some(BinOpKind::Mul),
        Token::Slash => Some(BinOpKind::Div),
        Token::Percent => Some(BinOpKind::Modulo),
        Token::Caret => Some(BinOpKind::Pow),
        _ => None,
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, CalcError> {
        let expr = self.parse_expression(0)?;
        if self.pos < self.tokens.len() {
            return Err(CalcError::ParseError(format!(
                "unexpected token at position {}",
                self.pos
            )));
        }
        Ok(expr)
    }

    /// Pratt parser: parse expression with given minimum precedence.
    fn parse_expression(&mut self, min_prec: u8) -> Result<Expr, CalcError> {
        let mut left = self.parse_unary()?;

        loop {
            let token = self.peek();
            let op = match token {
                Some(t) => match token_to_binop(t) {
                    Some(op) => op,
                    None => break,
                },
                None => break,
            };

            let prec = precedence(&op);
            if prec < min_prec {
                break;
            }

            self.advance();

            let next_min_prec = if op == BinOpKind::Pow {
                prec
            } else {
                prec + 1
            };

            let right = self.parse_expression(next_min_prec)?;
            left = Expr::binary(left, op, right);
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, CalcError> {
        if self.matches(&Token::Minus) {
            let expr = self.parse_unary()?;
            return Ok(Expr::unary_minus(expr));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, CalcError> {
        match self.peek().cloned() {
            Some(Token::Number(n)) => {
                self.advance();
                Ok(Expr::number(n))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression(0)?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Some(_) => Err(CalcError::ParseError("unexpected token".into())),
            None => Err(CalcError::ParseError("unexpected end of input".into())),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn matches(&mut self, expected: &Token) -> bool {
        match self.peek() {
            Some(t) if t == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), CalcError> {
        match self.peek() {
            Some(t) if t == expected => {
                self.advance();
                Ok(())
            }
            Some(t) => Err(CalcError::ParseError(format!(
                "expected {:?}, found {:?}",
                expected, t
            ))),
            None => Err(CalcError::ParseError("unexpected end of input".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Expr {
        let tokens = crate::token::Lexer::new(input).tokenize().unwrap();
        Parser::new(tokens).parse().unwrap()
    }

    #[test]
    fn parse_simple_add() {
        let expr = parse("1 + 2");
        assert_eq!(
            expr,
            Expr::binary(Expr::number(1.0), BinOpKind::Add, Expr::number(2.0))
        );
    }

    #[test]
    fn parse_precedence_mul_before_add() {
        let expr = parse("1 + 2 * 3");
        assert_eq!(
            expr,
            Expr::binary(
                Expr::number(1.0),
                BinOpKind::Add,
                Expr::binary(Expr::number(2.0), BinOpKind::Mul, Expr::number(3.0))
            )
        );
    }

    #[test]
    fn parse_parentheses() {
        let expr = parse("(1 + 2) * 3");
        assert_eq!(
            expr,
            Expr::binary(
                Expr::binary(Expr::number(1.0), BinOpKind::Add, Expr::number(2.0)),
                BinOpKind::Mul,
                Expr::number(3.0),
            )
        );
    }

    #[test]
    fn parse_unary_minus() {
        let expr = parse("-5");
        assert_eq!(expr, Expr::unary_minus(Expr::number(5.0)));
    }

    #[test]
    fn parse_unary_minus_expression() {
        let expr = parse("-(1 + 2)");
        assert_eq!(
            expr,
            Expr::unary_minus(Expr::binary(
                Expr::number(1.0),
                BinOpKind::Add,
                Expr::number(2.0)
            ))
        );
    }

    #[test]
    fn parse_power_right_associative() {
        let expr = parse("2 ^ 3 ^ 2");
        assert_eq!(
            expr,
            Expr::binary(
                Expr::number(2.0),
                BinOpKind::Pow,
                Expr::binary(Expr::number(3.0), BinOpKind::Pow, Expr::number(2.0))
            )
        );
    }
}
