use crate::token::Token;

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    VariableDeclaration { name: String, value: Box<AstNode> },
    BinaryExpression{ left: Box<AstNode>, operator: Token, right: Box<AstNode> },
    NumberLiteral(i64),
    StringLiteral(String),
    Identifier(String),
    PrintStatement(Box<AstNode>),
    ArrayLiteral(Vec<AstNode>), 
    DictionaryLiteral(Vec<(String, AstNode)>),
    IndexExpression(Box<AstNode>, Box<AstNode>),
    Bool(bool),
    Block(Vec<AstNode>),
    IfStatement {
        condition: Box<AstNode>,
        consequence: Box<AstNode>,
        alternative: Option<Box<AstNode>>,
    },
    WhileStatement {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    Break,
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
    FunctionCall {
        function: Box<AstNode>,
        arguments: Vec<AstNode>,
    },
    Return(Option<Box<AstNode>>),

}