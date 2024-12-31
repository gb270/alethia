use std::fmt;
use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div};
use crate::ast::AstNode;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Array(Vec<Value>),
    Dictionary(HashMap<String, Value>),
    Bool(bool),
    Function {
        name: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
    Nil, // for functions that don't return anything
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Value::Dictionary(dict) => {
                let pairs: Vec<String> = dict.iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v))
                    .collect();

                write!(f, "{{{}}}", pairs.join(", "))    
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            _ => panic!("Cannot display what you are trying to display."),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => Value::String(a + &b),
            (Value::String(a), Value::Number(b)) => Value::String(a + &b.to_string()),
            (Value::Number(a), Value::String(b)) => Value::String(a.to_string() + &b),
            (Value::String(a), Value::Nil) | (Value::Nil, Value::String(a)) => {
                Value::String(a + "nil")
            },
            _ => panic!("Cannot add in this instance!"),
        }
    }
}


impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("Cannot subtract non-numeric values"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                if n < 0 {
                    panic!("Cannot multiply string by negative number");
                }
                Value::String(s.repeat(n as usize))
            }
            _ => panic!("Cannot multiply two strings"),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0 {
                    panic!("Division by zero");
                }
                Value::Number(a / b)
            }
            _ => panic!("Cannot divide non-numeric values"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
        }
    }
}



