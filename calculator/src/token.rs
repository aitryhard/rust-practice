use crate::error::CalcError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CalcError> {
        let mut tokens = Vec::new();
        while self.pos < self.chars.len() {
            let ch = self.current();
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.advance();
                }
                '^' => {
                    tokens.push(Token::Caret);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }
                c if c.is_ascii_digit() || c == '.' => {
                    tokens.push(self.lex_number()?);
                }
                _ => {
                    return Err(CalcError::ParseError(format!(
                        "unexpected character '{}' at position {}",
                        ch,
                        self.pos
                    )));
                }
            }
        }
        Ok(tokens)
    }

    fn lex_number(&mut self) -> Result<Token, CalcError> {
        let start = self.pos;
        let mut dot_count = 0;

        while self.pos < self.chars.len() {
            let ch = self.current();
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    return Err(CalcError::ParseError(
                        "number contains multiple decimal points".into(),
                    ));
                }
                self.advance();
            } else {
                break;
            }
        }

        let num_str: String = self.chars[start..self.pos].iter().collect();
        let value = num_str
            .parse::<f64>()
            .map_err(|_| CalcError::ParseError(format!("invalid number '{}'", num_str)))?;
        Ok(Token::Number(value))
    }

    #[inline]
    fn current(&self) -> char {
        self.chars[self.pos]
    }

    #[inline]
    fn advance(&mut self) {
        self.pos += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_simple_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(42.0)]);
    }

    #[test]
    fn lex_float() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(3.14)]);
    }

    #[test]
    fn lex_expression() {
        let mut lexer = Lexer::new("1 + 2 * (3 - 4)");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::Number(2.0),
                Token::Star,
                Token::LParen,
                Token::Number(3.0),
                Token::Minus,
                Token::Number(4.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn lex_negative_number() {
        let mut lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Minus, Token::Number(5.0)]);
    }

    #[test]
    fn lex_power() {
        let mut lexer = Lexer::new("2 ^ 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(2.0), Token::Caret, Token::Number(3.0)]
        );
    }

    #[test]
    fn lex_error_multiple_dots() {
        let mut lexer = Lexer::new("1.2.3");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }
}
