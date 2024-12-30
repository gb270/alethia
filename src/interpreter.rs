use std::collections::HashMap;
use crate::ast::AstNode;
use crate::token::Token;
use crate::value::Value;

pub struct Interpreter {
    pub variables: HashMap<String, Value>,
    scopes: Vec<HashMap<String, Value>>,
}

#[derive(Debug)]
pub enum InterpreterError {
    Break,
    Return(Value),
    Error(String),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            scopes : vec! [HashMap::new()],
        }
    }

    fn is_truthy(&self, value: &Value) -> Result<bool, InterpreterError> {
        match value {
            Value::Bool(b) => Ok(*b),
            Value::String(s) if s == "true" || s == "false" => Ok(s == "true"),
            _ => Err(InterpreterError::Error(format!("Cannot convert {:?} to boolean", value)))
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn set_variable(&mut self, name: String, value: Value) {
        // Scope search same as in get_variable
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return;
            }
        }
    
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        } else {
            self.variables.insert(name, value);
        }
    }
    

    fn get_variable(&self, name: &str) -> Option<Value> {
        // search scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        // if not just return global scope
        self.variables.get(name).cloned()
    }

    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, InterpreterError> {
        match node {
            AstNode::Program(statements) => {
                let mut last_value = Value::Number(0);
                for statement in statements {
                    match self.evaluate(statement) {
                        Ok(value) => last_value = value,
                        Err(InterpreterError::Break) => return Err(InterpreterError::Error("Break statement outside of loop".to_string())),
                        Err(e) => return Err(e),
                    }
                }
                Ok(last_value)
            }
            AstNode::VariableDeclaration { name, value } => {
                let value = self.evaluate(value)?;
                self.variables.insert(name.clone(), value.clone());
                Ok(value)
            }
            AstNode::PrintStatement(expression) => {
                let value = self.evaluate(expression)?;
                // trying to avoid printing Value::Nil
                if value != Value::Nil {
                    println!("{}", value);
                }
                Ok(Value::Number(0))
            }
            AstNode::BinaryExpression { left, operator, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                match operator {
                    Token::Plus => Ok(left_val + right_val),
                    Token::Minus => Ok(left_val - right_val),
                    Token::Multiply => Ok(left_val * right_val),
                    Token::Divide => {
                        if right_val == Value::Number(0) {
                            Err(InterpreterError::Error("Division by zero".to_string()))
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    Token::Equal => {
                        let is_equal = left_val == right_val;
                        Ok(Value::String(is_equal.to_string()))
                    }
                    Token::LessThan => {
                        if let (Value::Number(lhs), Value::Number(rhs)) = (&left_val, &right_val) {
                            Ok(Value::String((lhs < rhs).to_string()))
                        } else {
                            Err(InterpreterError::Error("Cannot compare non-numeric values with <".to_string()))
                        }
                    }
                    Token::GreaterThan => {
                        if let (Value::Number(lhs), Value::Number(rhs)) = (&left_val, &right_val) {
                            Ok(Value::String((lhs > rhs).to_string()))
                        } else {
                            Err(InterpreterError::Error("Cannot compare non-numeric values with >".to_string()))
                        }
                    }
                    Token::And => {
                        match (&left_val, &right_val) {
                            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(*lhs && *rhs)),
                            (Value::String(ls), Value::String(rs)) if ls == "true" || ls == "false" => {
                                let lhs = ls == "true";
                                let rhs = rs == "true";
                                Ok(Value::Bool(lhs && rhs))
                            }
                            _ => {
                                println!("DEBUG: And operation failed. Left: {:?}, Right: {:?}", left_val, right_val);
                                Err(InterpreterError::Error("Cannot apply `and` to non-boolean values".to_string()))
                            }
                        }
                    }
                    Token::Or => {
                        match (&left_val, &right_val) {
                            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(*lhs || *rhs)),
                            (Value::String(ls), Value::String(rs)) if ls == "true" || ls == "false" => {
                                let lhs = ls == "true";
                                let rhs = rs == "true";
                                Ok(Value::Bool(lhs || rhs))
                            }
                            _ => {
                                println!("DEBUG: Or operation failed. Left: {:?}, Right: {:?}", left_val, right_val);
                                Err(InterpreterError::Error("Cannot apply `or` to non-boolean values".to_string()))
                            }
                        }
                    }
                    _ => Err(InterpreterError::Error(format!("Unsupported operator: {:?}", operator))),
                }
            }
            AstNode::Bool(value) => Ok(Value::Bool(*value)),
            AstNode::StringLiteral(value) => Ok(Value::String(value.clone())),
            AstNode::NumberLiteral(value) => Ok(Value::Number(*value)),
            AstNode::Identifier(name) => {
                self.get_variable(name)
                    .ok_or_else(|| InterpreterError::Error(format!("Undefined variable: {}", name)))
            }
            AstNode::ArrayLiteral(elements) => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.evaluate(e)?);
                }
                Ok(Value::Array(vals))
            }
            AstNode::IndexExpression(array_node, index_node) => {
                let array_val = self.evaluate(array_node)?;
                let index_val = self.evaluate(index_node)?;
                match (array_val, index_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        if idx < 0 || idx as usize >= arr.len() {
                            return Err(InterpreterError::Error(format!("Array index {} out of bounds", idx)));
                        }
                        Ok(arr[idx as usize].clone())
                    }
                    (Value::Dictionary(dict), Value::String(key)) => {
                        dict.get(&key)
                            .cloned()
                            .ok_or_else(|| InterpreterError::Error(format!("Key '{}' not found in dictionary", key)))
                    }
                    _ => Err(InterpreterError::Error("Indexing error: expected array or dictionary with correct key type".to_string())),
                }
            }
            AstNode::DictionaryLiteral(pairs) => {
                let mut map = HashMap::new();
                for (key, expr) in pairs {
                    let value = self.evaluate(expr)?;
                    map.insert(key.clone(), value);
                }
                Ok(Value::Dictionary(map))
            }
            AstNode::IfStatement { condition, consequence, alternative } => {
                let condition_value = self.evaluate(condition)?;
                match self.is_truthy(&condition_value)? {
                    true => self.evaluate(consequence),
                    false => {
                        if let Some(alt) = alternative {
                            self.evaluate(alt)
                        } else {
                            Ok(Value::Number(0))
                        }
                    }
                }
            }
            AstNode::WhileStatement { condition, body } => {
                loop {
                    let cond_val = self.evaluate(condition)?;
                    if !self.is_truthy(&cond_val)? {
                        break;
                    }

                    match self.evaluate(body) {
                        Ok(_) => continue,
                        Err(InterpreterError::Break) => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(Value::Number(0))
            }
            AstNode::Break => Err(InterpreterError::Break),
            AstNode::FunctionDeclaration { name, params, body } => {
                let func = Value::Function { 
                    name: name.clone(), 
                    params: params.clone(), 
                    body: body.clone(),
                };
                self.set_variable(name.clone(), func.clone());
                Ok(func)
            }
            AstNode::FunctionCall { function, arguments } => {
                let func = self.evaluate(function)?;
                match func {
                    Value::Function { params, body, .. } => {
                        // create new scope for function execution
                        self.push_scope();

                        // bind parameters to arguments
                        for (param, arg) in params.iter().zip(arguments) {
                            let arg_val = self.evaluate(arg)?;
                            self.set_variable(param.clone(), arg_val);
                        }

                        // execute function body
                        let result = match self.evaluate(&body) {
                            Ok(val) => val,
                            Err(InterpreterError::Return(val)) => val,
                            Err(e) => {
                                self.pop_scope();
                                return Err(e);
                            }
                        };

                        self.pop_scope();
                        Ok(result)
                    }
                    _ => Err(InterpreterError::Error("Not a function".to_string())),
                }
            }

            AstNode::Return(expr) => {
                let value = if let Some(expr) = expr {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                Err(InterpreterError::Return(value))
            }
            AstNode::Block(statements) => {
                let mut last_value = Value::Number(0);
                for statement in statements {
                    match self.evaluate(statement) {
                        Ok(value) => last_value = value,
                        Err(InterpreterError::Break) => return Err(InterpreterError::Break),
                        Err(InterpreterError::Return(val)) => return Err(InterpreterError::Return(val)),
                        Err(e) => return Err(e),
                    }
                }
                Ok(last_value)
            }
        }
    }
}