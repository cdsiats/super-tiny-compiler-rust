use std::char;

use crate::tokens::Token;

pub struct Lexer {
    pub source: String,
    pub current: usize
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            source: input,
            current: 0
        }
    }
    
    fn advance(&mut self) {
        self.current += 1;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        
        while self.current < self.source.len() {
            let char = &self.source.chars().nth(self.current).unwrap();

            match char {
                '(' => {
                    tokens.push(Token { 
                        kind: "paren".to_string(), 
                        value: "(".to_string(),
                    });
                    self.advance();
                    continue;
                },
                ')' => {
                    tokens.push(Token { 
                        kind: "paren".to_string(), 
                        value: ")".to_string(), 
                    });
                    self.advance();
                    continue;
                },
                char if char.is_whitespace() => {
                    self.advance();
                    continue;
                }
                _ => todo!("Handle catch all.")
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens;

    use super::*;

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::new("\n\t".to_string());
        let tokens = lexer.tokenize();

        assert!(tokens.is_empty(),);
    }

    #[test]
    fn test_parens() {
        let mut lexer = Lexer::new("()".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, "paren");
        assert_eq!(tokens[0].value, "(");
        assert_eq!(tokens[1].kind, "paren");
        assert_eq!(tokens[1].value, ")");
    }
}