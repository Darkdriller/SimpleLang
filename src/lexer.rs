
#[derive(Clone,PartialEq,Debug)]
pub enum Tokentype {
    INT,
    IF,
    ELSE,
    IDENTIFIER,
    NUMBER,
    PLUS,
    MINUS,
    EQ,
    EQUAL,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,
    END,
}
#[derive(Clone,PartialEq,Debug)]
pub struct Token {
    pub typ: Tokentype,
    pub value: String,
}

pub struct Lexer {
    source: String,
    pos: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.source.len() {
            let cur = self.source.as_bytes()[self.pos] as char;
            if cur.is_whitespace() {
                self.pos += 1;
                continue;
            }

            if cur.is_alphabetic() {
                let mut identifier = String::new();
                while self.pos < self.source.len() && self.source.as_bytes()[self.pos].is_ascii_alphanumeric() {                    
                    let ch = self.source.as_bytes()[self.pos] as char; // Get the current character
                    identifier.push(ch);
                            self.pos += 1;
                        if self.pos >= self.source.len() {
                        break;
                    }
                }
                match identifier.as_str() {
                    "int" => tokens.push(Token { typ: Tokentype::INT, value: "int".to_string() }),
                    "if" => tokens.push(Token { typ: Tokentype::IF, value: "if".to_string() }),
                    "else" => tokens.push(Token { typ: Tokentype::ELSE, value: "else".to_string() }),
                    _ => tokens.push(Token { typ: Tokentype::IDENTIFIER, value: identifier }),
                }
            } else if cur.is_digit(10) {
                let mut number = String::new();
                while self.pos < self.source.len() && self.source.as_bytes()[self.pos].is_ascii_digit() {
                    number.push(self.source.as_bytes()[self.pos] as char);
                    self.pos += 1;
                }
                tokens.push(Token { typ: Tokentype::NUMBER, value: number });
            } else {
                match cur {
                    '+' => tokens.push(Token { typ: Tokentype::PLUS, value: "+".to_string() }),
                    '-' => tokens.push(Token { typ: Tokentype::MINUS, value: "-".to_string() }),
                    '=' => {
                                if self.pos + 1 < self.source.len() && self.source.as_bytes()[self.pos + 1] as char == '=' {
                                    tokens.push(Token { typ: Tokentype::EQUAL, value: "==".to_string() });
                                    self.pos += 1;
                                } else {
                                    tokens.push(Token { typ: Tokentype::EQ, value: "=".to_string() });
                                }
                            }
                    '(' => tokens.push(Token { typ: Tokentype::LPAREN, value: "(".to_string() }),
                    ')' => tokens.push(Token { typ: Tokentype::RPAREN, value: ")".to_string() }),
                    '{' => tokens.push(Token { typ: Tokentype::LBRACE, value: "{".to_string() }),
                    '}' => tokens.push(Token { typ: Tokentype::RBRACE, value: "}".to_string() }),
                    ';' => tokens.push(Token { typ: Tokentype::SEMICOLON, value: ";".to_string() }),
                    _ => panic!("Unexpected character: {}", cur),
                }
                self.pos += 1;
            }
        }
        tokens.push(Token { typ: Tokentype::END, value: "".to_string() });
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_identifiers() {
        let source = String::from("int a = 5;");
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let expected_tokens = vec![
            Token { typ: Tokentype::INT, value: "int".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "a".to_string() },
            Token { typ: Tokentype::EQ, value: "=".to_string() },
            Token { typ: Tokentype::NUMBER, value: "5".to_string() },
            Token { typ: Tokentype::SEMICOLON, value: ";".to_string() },
            Token { typ: Tokentype::END, value: "".to_string() },
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_tokenize_if_else() {
        let source = String::from("if (x == y) { y = y + 1; } else { y = y - 1; }");
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let expected_tokens = vec![
            Token { typ: Tokentype::IF, value: "if".to_string() },
            Token { typ: Tokentype::LPAREN, value: "(".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "x".to_string() },
            Token { typ: Tokentype::EQUAL, value: "==".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "y".to_string() },
            Token { typ: Tokentype::RPAREN, value: ")".to_string() },
            Token { typ: Tokentype::LBRACE, value: "{".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "y".to_string() },
            Token { typ: Tokentype::EQ, value: "=".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "y".to_string() },
            Token { typ: Tokentype::PLUS, value: "+".to_string() },
            Token { typ: Tokentype::NUMBER, value: "1".to_string() },
            Token { typ: Tokentype::SEMICOLON, value: ";".to_string() },
            Token { typ: Tokentype::RBRACE, value: "}".to_string() },
            Token { typ: Tokentype::ELSE, value: "else".to_string() },
            Token { typ: Tokentype::LBRACE, value: "{".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "y".to_string() },
            Token { typ: Tokentype::EQ, value: "=".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "y".to_string() },
            Token { typ: Tokentype::MINUS, value: "-".to_string() },
            Token { typ: Tokentype::NUMBER, value: "1".to_string() },
            Token { typ: Tokentype::SEMICOLON, value: ";".to_string() },
            Token { typ: Tokentype::RBRACE, value: "}".to_string() },
            Token { typ: Tokentype::END, value: "".to_string() },
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let source = String::from("int x = 42 + y;");
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let expected_tokens = vec![
            Token { typ: Tokentype::INT, value: "int".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "x".to_string() },
            Token { typ: Tokentype::EQ, value: "=".to_string() },
            Token { typ: Tokentype::NUMBER, value: "42".to_string() },
            Token { typ: Tokentype::PLUS, value: "+".to_string() },
            Token { typ: Tokentype::IDENTIFIER, value: "y".to_string() },
            Token { typ: Tokentype::SEMICOLON, value: ";".to_string() },
            Token { typ: Tokentype::END, value: "".to_string() },
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
