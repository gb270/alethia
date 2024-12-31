use crate::ast::AstNode;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    pub fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    pub fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    pub fn consume(&mut self, expected: &Token) -> Result<(), String> {
        if self.current_token() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, but found {:?}",
                expected,
                self.current_token()
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<AstNode, String> {
        let mut statements = Vec::new();
    
        while self.current_token() != &Token::Eof {
            if *self.current_token() == Token::Semicolon {
                self.advance();
                continue;
            }
    
            let stmt = self.parse_statement()?;
            let is_block_or_function = matches!(
                stmt,
                AstNode::IfStatement { .. }
                    | AstNode::WhileStatement { .. }
                    | AstNode::FunctionDeclaration { .. }
            );
    
            statements.push(stmt);
    
            if is_block_or_function {
                // semicolon is option after blocks or functions
                if *self.current_token() == Token::Semicolon {
                    self.advance();
                }
            } else {
                // semicolon is required after everything else
                if *self.current_token() != Token::Semicolon {
                    return Err(format!(
                        "Expected Semicolon, but found {:?}",
                        self.current_token()
                    ));
                }
                self.advance();
            }
        }
    
        Ok(AstNode::Program(statements))
    }
    

    pub fn parse_statement(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::Let => {
                self.advance();
    
                let name = match self.current_token() {
                    Token::Identifier(ref id) => {
                        let name = id.clone();
                        self.advance();
                        name
                    }
                    _ => return Err(format!("Expected identifier after 'let', found {:?}", self.current_token())),
                };
    
                match self.current_token() {
                    Token::Assign => self.advance(), 
                    _ => return Err(format!("Expected '=' after identifier in let statement, found {:?}", self.current_token())),
                }
    
                let value = self.parse_expression()?;
                Ok(AstNode::VariableDeclaration {
                    name,
                    value: Box::new(value),
                })
            }
            Token::Identifier(ref name) => {
                // Check if the next token is '=' for assignment
                if let Some(Token::Assign) = self.tokens.get(self.position + 1) {
                    let name = name.clone();
                    self.advance(); 
                    self.advance(); 
                    let value = self.parse_expression()?;
                    Ok(AstNode::VariableDeclaration {
                        name,
                        value: Box::new(value),
                    })
                } else {
                    // Otherwise, treat as an expression
                    self.parse_expression()
                }
            }
            Token::Print => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(AstNode::PrintStatement(Box::new(expr)))
            }
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::Break => {
                self.advance();
                Ok(AstNode::Break)
            }
            Token::Func => self.parse_function_declaration(),
            Token::Return => self.parse_return_statement(),
            _ => Err(format!("Unexpected token in statement: {:?}", self.current_token())),
        }
    }
    


    pub fn parse_expression(&mut self) -> Result<AstNode, String> {
        self.parse_or_expression()
    }

    fn parse_or_expression(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_and_expression()?;

        while matches!(self.current_token(), Token::Or) {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_and_expression()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and_expression(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current_token(), Token::And) {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_comparison()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_sum()?;

        while matches!(self.current_token(), Token::Equal | Token::LessThan | Token::GreaterThan) {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_sum()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_sum(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_product()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_product()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_product(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_factor()?;

        while matches!(self.current_token(), Token::Multiply | Token::Divide) {
            let operator = self.current_token().clone();
            self.advance();
            let right = self.parse_factor()?;
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse_factor(&mut self) -> Result<AstNode, String> {
        match self.current_token().clone() {
            Token::LeftBracket => self.parse_array_literal(),
            Token::LeftCurly => self.parse_dictionary_literal(),
            Token::Identifier(name) => {
                self.advance();
                // Handle indexing or function calls
                if *self.current_token() == Token::LeftBracket {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(&Token::RightBracket)?;
                    Ok(AstNode::IndexExpression(
                        Box::new(AstNode::Identifier(name)),
                        Box::new(index),
                    ))
                } else if *self.current_token() == Token::LeftParen {
                    self.advance();
                    let mut arguments = Vec::new();
    
                    if *self.current_token() != Token::RightParen {
                        loop {
                            let arg = self.parse_expression()?;
                            arguments.push(arg);
    
                            match self.current_token() {
                                Token::Comma => {
                                    self.advance();
                                    continue;
                                }
                                Token::RightParen => break,
                                _ => {
                                    return Err(format!(
                                        "Expected ',' or ')', found {:?}",
                                        self.current_token()
                                    ));
                                }
                            }
                        }
                    }
                    self.consume(&Token::RightParen)?;
                    Ok(AstNode::FunctionCall {
                        function: Box::new(AstNode::Identifier(name)),
                        arguments,
                    })
                } else {
                    Ok(AstNode::Identifier(name))
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(&Token::RightParen)?;
                Ok(expr)
            }
            Token::Nil => {
                self.advance();
                Ok(AstNode::Nil)
            }
            
            Token::Number(n) => {
                self.advance();
                Ok(AstNode::NumberLiteral(n))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(AstNode::StringLiteral(s))
            }
            Token::True => {
                self.advance();
                Ok(AstNode::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(AstNode::Bool(false))
            }
            _ => Err(format!("Unexpected token in factor: {:?}", self.current_token())),
        }
    }
    
    fn parse_array_literal(&mut self) -> Result<AstNode, String> {
        self.consume(&Token::LeftBracket)?;

        let mut elements = Vec::new();

        if *self.current_token() == Token::RightBracket {
            self.advance();
            return Ok(AstNode::ArrayLiteral(elements));
        }

        loop {
            let element = self.parse_expression()?;
            elements.push(element);

            match self.current_token() {
                Token::Comma => {
                    self.advance();
                    if *self.current_token() == Token::RightBracket {
                        self.advance();
                        break;
                    }
                }
                Token::RightBracket => {
                    self.advance();
                    break;
                }
                _ => return Err(format!("Expected ',' or ']', found {:?}", self.current_token())),
            }
        }

        Ok(AstNode::ArrayLiteral(elements))
    }

    fn parse_dictionary_literal(&mut self) -> Result<AstNode, String> {
        self.consume(&Token::LeftCurly)?;

        let mut pairs = Vec::new();

        if *self.current_token() == Token::RightCurly {
            self.advance();
            return Ok(AstNode::DictionaryLiteral(pairs));
        }

        loop {
            let key = match self.current_token().clone() {
                Token::Identifier(name) => {
                    self.advance();
                    name
                }
                Token::StringLiteral(s) => {
                    self.advance();
                    s
                }
                other => {
                    return Err(format!(
                        "Expected identifier or string literal as dictionary key, found {:?}",
                        other
                    ))
                }
            };

            self.consume(&Token::Colon)?;
            let value = self.parse_expression()?;
            pairs.push((key, value));

            match self.current_token() {
                Token::Comma => {
                    self.advance();
                    if *self.current_token() == Token::RightCurly {
                        self.advance();
                        break;
                    }
                }
                Token::RightCurly => {
                    self.advance();
                    break;
                }
                other => {
                    return Err(format!(
                        "Expected ',' or '}}' in dictionary literal, found {:?}",
                        other
                    ))
                }
            }
        }

        Ok(AstNode::DictionaryLiteral(pairs))
    }

    fn parse_block(&mut self) -> Result<AstNode, String> {
        self.consume(&Token::LeftCurly)?;
        
        let mut statements = Vec::new();
        
        while self.current_token() != &Token::RightCurly && self.current_token() != &Token::Eof {
            if *self.current_token() == Token::Semicolon {
                self.advance();
                continue;
            }
            
            if *self.current_token() == Token::RightCurly {
                break;
            }
            
            let stmt = self.parse_statement()?;
            let is_block = matches!(&stmt, AstNode::IfStatement { .. } | AstNode::WhileStatement { .. });
            statements.push(stmt);
            
            // Handle semicolons based on statement type
            if is_block {
                // Optional semicolon after blocks
                if *self.current_token() == Token::Semicolon {
                    self.advance();
                }
            } else {
                // Require semicolons after non-block statements unless followed by }
                if *self.current_token() != Token::RightCurly {
                    if *self.current_token() != Token::Semicolon {
                        return Err(format!("Expected Semicolon or }}, found {:?}", self.current_token()));
                    }
                    self.advance();
                }
            }
        }
        
        self.consume(&Token::RightCurly)?;
        Ok(AstNode::Block(statements))
    }

    fn parse_if_statement(&mut self) -> Result<AstNode, String> {
        self.consume(&Token::If)?;
        
        let condition = self.parse_expression()?;
        let consequence = self.parse_block()?;
        
        let alternative = if *self.current_token() == Token::Else {
            self.advance();
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };
        
        Ok(AstNode::IfStatement {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        })
    }
    
    fn parse_while_statement(&mut self) -> Result<AstNode, String> {
        self.consume(&Token::While)?;
        
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        
        Ok(AstNode::WhileStatement {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }


    fn parse_function_declaration(&mut self) -> Result<AstNode, String> {
        self.advance();
        
        // Get function name
        let name = match self.current_token() {
            Token::Identifier(ref id) => {
                let name = id.clone();
                self.advance();
                name
            }
            _ => return Err(format!("Expected identifier after 'func', found {:?}", self.current_token())),
        };

        // Parse parameter list
        self.consume(&Token::LeftParen)?;
        let mut params = Vec::new();
        
        if *self.current_token() != Token::RightParen {
            loop {
                match self.current_token() {
                    Token::Identifier(ref param) => {
                        params.push(param.clone());
                        self.advance();
                    }
                    _ => return Err(format!("Expected parameter name, found {:?}", self.current_token())),
                }

                match self.current_token() {
                    Token::Comma => {
                        self.advance();
                        continue;
                    }
                    Token::RightParen => break,
                    _ => return Err(format!("Expected ',' or ')', found {:?}", self.current_token())),
                }
            }
        }
        
        self.consume(&Token::RightParen)?;
        
        // Parse function body
        let body = self.parse_block()?;
        
        Ok(AstNode::FunctionDeclaration {
            name,
            params,
            body: Box::new(body),
        })
    }

    fn parse_return_statement(&mut self) -> Result<AstNode, String> {
        self.advance();
        
        // Check if there's a value to return
        let expr = if *self.current_token() != Token::Semicolon {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        Ok(AstNode::Return(expr))
    }

}

