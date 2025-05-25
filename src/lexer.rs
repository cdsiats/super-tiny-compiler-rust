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
                        value: char.to_string(),
                    });
                    self.advance();
                    continue;
                },
                _ => todo!("Handle catch all.")
            }
        }

        tokens
    }
}