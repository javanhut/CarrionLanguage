use crate::ast::{BlockStatement, Identifier};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    List(Vec<Object>),
    Dict(HashMap<Object, Object>),
    ReturnValue(Box<Object>),
    Function(Function),
    Error(String),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl std::hash::Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Object::Integer(i) => i.hash(state),
            Object::Boolean(b) => b.hash(state),
            Object::String(s) => s.hash(state),

            _ => "UNHASHABLE".hash(state),
        }
    }
}
impl Eq for Object {}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Float(val) => write!(f, "{}", val),
            Object::Boolean(val) => write!(f, "{}", val),
            Object::String(val) => write!(f, "{}", val),
            Object::List(items) => {
                let parts: Vec<String> = items.iter().map(|i| i.to_string()).collect();
                write!(f, "[{}]", parts.join(", "))
            }
            Object::Dict(map) => {
                let parts: Vec<String> = map.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", parts.join(", "))
            }
            Object::ReturnValue(val) => write!(f, "{}", val),
            Object::Function(_) => write!(f, "[Function]"),
            Object::Error(msg) => write!(f, "Error: {}", msg),
            Object::None => write!(f, "none"),
        }
    }
}
