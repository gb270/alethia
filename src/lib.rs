mod token;
mod ast;
mod lexer;
mod parser;
mod value;
mod interpreter;

pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use parser::Parser;
pub use ast::AstNode;
pub use token::Token;
pub use interpreter::InterpreterError;

pub fn run_source(source: String) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token == Token::Eof { 
            break; 
        }
    }
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.evaluate(&ast).map_err(|e| match e {
        InterpreterError::Break => "Break statement outside of loop".to_string(),
        InterpreterError::Return(val) => format!("Return statement outside of function: {:?}", val),
        InterpreterError::Error(msg) => msg,
    })?;
    Ok(())
}

pub fn run_line(source: String, interpreter: &mut Interpreter) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let mut ast_nodes = Vec::new();

    while parser.current_token() != &Token::Eof {
        match parser.parse_statement() {
            Ok(ast) => ast_nodes.push(ast),
            Err(err) => return Err(format!("Parsing error: {}", err)),
        }
    }

    let program_ast = AstNode::Program(ast_nodes);

    match interpreter.evaluate(&program_ast) {
        Ok(_) => Ok(()),
        Err(InterpreterError::Break) => Err("Break statement outside of loop".to_string()),
        Err(InterpreterError::Return(val)) => Err(format!("Return statement outside of function: {:?}", val)),
        Err(InterpreterError::Error(msg)) => Err(format!("Runtime error: {}", msg)),
    }
}