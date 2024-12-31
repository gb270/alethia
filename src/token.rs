#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,                // Keyword: let
    Print,              // Keyword: print
    If,                 // Keyword: if
    Else,               // Keyword: else
    While,              // Keyword: while
    Break,              // Keyword: break
    Func,               // Keyword: func
    Return,             // Keyword: return
    Identifier(String), // Identifiers: variable names
    Number(i64),        // Numbers
    StringLiteral(String), // For string literals
    Assign,             // '=' symbol
    Equal,              // '==' symbol
    Plus,               // '+' symbol
    Minus,              // '-' symbol
    Multiply,           // '*' symbol
    Divide,             // '/' symbol
    LessThan,           // '<' symbol
    GreaterThan,        // '>' symbol
    LeftParen,          // '(' symbol
    RightParen,         // ')' symbol
    LeftBracket,        // '[' symbol used for array
    RightBracket,       // ']' symbol used for array
    LeftCurly,          // '{' symbol used for dictionary and blocks
    RightCurly,         // '}' symbol used for dictionary and blocks
    Colon,              // ':'
    Comma,              // ','
    Semicolon,          // ';'
    True,               // Keyword: true
    False,              // Keyword: false
    Or,                 // Keyword: or
    And,                // Keyword: and
    Nil,                // Keyword: nil
    Eof,                // End of file/input
    Unknown(char),      // Unknown character
}