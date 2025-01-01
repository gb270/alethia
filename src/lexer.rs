use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>, 
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let lexer = Self {
            position: 0,
            current_char: input.chars().next(),
            input,
        };
        lexer
    }

    pub fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    // peeks at the next character without consuming it
    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }


    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            match c {
                // skip whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }
                '=' => {
                    if self.peek() == Some('=') {
                        // move forward twice to consume both '='
                        self.advance();
                        self.advance(); 
                        return Token::Equal; 
                    }
                    self.advance(); 
                    return Token::Assign; 
                }
                '+' => {
                    self.advance();
                    return Token::Plus;
                }
                '-' => {
                    self.advance();
                    return Token::Minus;
                }
                '*' => {
                    self.advance();
                    return Token::Multiply;
                }
                '/' => {
                    self.advance();
                    return Token::Divide;
                }
                '<' => {
                    self.advance();
                    return Token::LessThan;
                }
                '>' => {
                    self.advance();
                    return Token::GreaterThan;
                }
                '(' => {
                    self.advance();
                    return Token::LeftParen;
                }
                ')' => {
                    self.advance();
                    return Token::RightParen;
                }
                '[' => {
                    self.advance();
                    return Token::LeftBracket;
                }
                ']' => {
                    self.advance();
                    return Token::RightBracket;
                }
                ',' => {
                    self.advance();
                    return Token::Comma;
                }
                '{' =>{
                    self.advance();
                    return Token::LeftCurly;
                }
                '}' =>{
                    self.advance();
                    return Token::RightCurly;
                }
                ':' =>{
                    self.advance();
                    return Token::Colon;
                }
                ';' =>{
                    self.advance();
                    return Token::Semicolon;
                }
                '"' => return self.string_literal(),
                '0'..='9' => return self.number(),
                'a'..='z' | 'A'..='Z' | '_' => return self.identifier_or_keyword().expect("Some issue going on with creating a string?"),

                _ => {
                    self.advance();
                    return Token::Unknown(c);
                }

            }
        }
        Token::Eof
    }

    pub fn string_literal(&mut self) -> Token {
        self.advance();
        let mut value = String::new();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                return Token::StringLiteral(value);
            } else {
                value.push(c);
                self.advance();
            }
        }
        // If we reach here, string wasn't closed properly
        // can also return an error
        Token::Unknown('"')
    }

    pub fn number(&mut self) -> Token {
        let mut number = String::new();
        let mut has_decimal = false;
    
        while let Some(c) = self.current_char {
            if c.is_numeric() {
                number.push(c);
                self.advance();
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(number.parse::<f64>().unwrap())
    }

    pub fn identifier_or_keyword(&mut self) -> Option<Token> {
        let mut identifier = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Some(match identifier.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "break" => Token::Break,
            "func" => Token::Func,
            "return" => Token::Return,
            "or" => Token::Or,
            "and" => Token::And,
            "nil" => Token::Nil,
            _ => Token::Identifier(identifier),
        })
    }
    

}


