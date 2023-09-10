use crate::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<Token> {
        if self.position < self.tokens.len() {
            Some(self.tokens[self.position])
        } else {
            None
        }
    }

    fn next_position(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    pub fn parse(&mut self) -> Result<i32, &'static str> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<i32, &'static str> {
        let mut result = self.parse_term()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::Add => {
                    self.next_position();
                    result += self.parse_term()?;
                }
                Token::Subtract => {
                    self.next_position();
                    result -= self.parse_term()?;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_term(&mut self) -> Result<i32, &'static str> {
        let mut result = self.parse_factor()?;

        while let Some(token) = self.current_token() {
            match token {
                Token::Multiply => {
                    self.next_position();
                    result *= self.parse_factor()?;
                }
                Token::Divide => {
                    self.next_position();
                    let divisor = self.parse_factor()?;
                    if divisor == 0 {
                        return Err("Division by zero");
                    }
                    result /= divisor;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<i32, &'static str> {
        match self.current_token() {
            Some(Token::Number(num)) => {
                self.next_position();
                Ok(num)
            }
            Some(Token::OpenBracket) => {
                self.next_position();
                let result = self.parse_expression()?;
                if self.current_token() == Some(Token::CloseBracket) {
                    self.next_position();
                    Ok(result)
                } else {
                    Err("Expected closing bracket")
                }
            }
            _ => Err("Expected number or opening bracket"),
        }
    }
}
