use std::char;

use crate::tokens::Token;

pub struct Lexer {
    pub source: Vec<char>,
    pub current: usize
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            source: input.chars().collect(),
            current: 0
        }
    }
    
    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&mut self) -> Option<char> {
        self.source.get(self.current).cloned()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        
        while self.current < self.source.len() {
            let char = self.source[self.current];

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
                },
                c if c.is_numeric() => {
                    let mut value = String::from("");
                    //push the first value
                    value.push(c.clone());
                    self.advance();
                    //while the next number is still numeric
                    //push the value
                    while let Some(char) = self.peek() {
                        if char.is_numeric() {
                            value.push(char);
                            self.advance();
                        } else { break;}
                    }

                    tokens.push(Token { 
                        kind: "number".to_string(), 
                        value: value,
                    });

                    continue;
                },
                '"' => {
                    let mut value = String::from("");
                    //skip opening quote
                    self.advance();

                    while let Some(next_char) = self.peek() {
                        if next_char == '"' {
                            self.advance();
                            break;
                        } else {
                            value.push(next_char);
                            self.advance();
                        }
                    }

                    tokens.push(Token { 
                        kind: "string".to_string(), 
                        value: value, 
                    });

                    continue;
                },
                c if c.is_alphabetic() => {
                    let mut value = String::from("");

                    value.push(c.clone());
                    self.advance();

                    while let Some(next_char) = self.peek() {
                        if next_char.is_alphabetic() {
                            value.push(next_char);
                            self.advance();
                        } else { break; }
                    }

                    tokens.push(Token { 
                        kind: "name".to_string(), 
                        value: value, 
                    });

                    continue;
                },
                char => panic!("Unknown character: {}", char)
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

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("1 2".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, "number");
        assert_eq!(tokens[0].value, "1".to_string());
        assert_eq!(tokens[1].kind, "number");
        assert_eq!(tokens[1].value, "2".to_string());
    }

    #[test]
    fn test_names() {
        let mut lexer = Lexer::new("add".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, "name");
        assert_eq!(tokens[0].value, "add");
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new("\"hello\"".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, "string");
        assert_eq!(tokens[0].value, "hello".to_string());
    }

    #[test]
    fn test_multiple_tokens() {
        let mut lexer = Lexer::new("(add 1 2)".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, "paren");
        assert_eq!(tokens[0].value, "(".to_string());
        assert_eq!(tokens[1].kind, "name");
        assert_eq!(tokens[1].value, "add".to_string());
        assert_eq!(tokens[2].kind, "number");
        assert_eq!(tokens[2].value, "1".to_string());
        assert_eq!(tokens[3].kind, "number");
        assert_eq!(tokens[3].value, "2".to_string());
        assert_eq!(tokens[4].kind, "paren");
        assert_eq!(tokens[4].value, ")".to_string());
    }

    #[test]
    #[should_panic]
    fn test_invalid_char() {
        let mut lexer = Lexer::new("@".to_string());
        lexer.tokenize();
        
    }
}