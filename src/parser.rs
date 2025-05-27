
use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Program {
        body: Vec<Node>,
    },
    Literal {
        _type: String,
        value: String,
    },
    CallExpression {
        name: String,
        params: Vec<Node>,
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// The parser takes in an array of tokens
// and produces an AST
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn current_token(&mut self) -> Option<Token> {
        return self.tokens.get(self.current).cloned();
    }

    pub fn parse_program(&mut self) -> Node {
        let mut body: Vec<Node> = Vec::new();

        while self.current < self.tokens.len() {
            body.push(self.parse_expression());
        }

        Node::Program { body }
    }

    pub fn parse_expression(&mut self) -> Node {
        let token = self.current_token().expect("Unexpected end of input");

        match token.kind.as_str() {
            "number" => {
                self.advance();
                
                Node::Literal { 
                    _type: "NumberLiteral".to_string(), 
                    value: token.value, 
                }
            },
            "string" => {
                self.advance();

                Node::Literal { 
                    _type: "StringLiteral".to_string(), 
                    value: token.value 
                }
            },
            "paren" if token.value == "(" => {
                self.advance();
                
                let name_token = self.current_token().expect("Expected function name");
                let name = name_token.value.clone();
                //skips the name token
                self.advance();

                let mut params: Vec<Node> = Vec::new();

                loop {
                    let current = self.current_token().expect("Expected token in call expression");

                    if current.kind == "paren" && current.value == ")" {
                        break;
                    }

                    params.push(self.parse_expression());
                }

                //skip closing paren
                self.advance();

                Node::CallExpression { 
                    name, 
                    params 
                }
            },
            _ => panic!("Unexpected token: {}", token.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_numbers() {
        let mut lexer = Lexer::new("1 2".to_string());
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program();

        let expected = Node::Program { 
            body: vec![
                Node::Literal { 
                    _type: "NumberLiteral".to_string(), 
                    value: "1".to_string(), 
                },
                Node::Literal { 
                    _type: "NumberLiteral".to_string(), 
                    value: "2".to_string(), 
                },
            ]
        };

        assert_eq!(ast, expected);
    }
}