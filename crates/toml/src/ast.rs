use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Table(HashMap<String, Value>),

    Conditional {
        condition: Expr,
        then_branch: Box<Value>,
        else_branch: Option<Box<Value>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(String),
    String(String),
    NotEquals(Box<Expr>, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Identifier(s) => write!(f, "{}", s),
            Expr::String(s) => write!(f, "{:?}", s),
            Expr::NotEquals(lhs, rhs) => write!(f, "{} != {}", lhs, rhs),
            Expr::Equals(lhs, rhs) => write!(f, "{} == {}", lhs, rhs),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{:?}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),

            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }

            Value::Table(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", k, v)?;
                }
                write!(f, "}}")
            }

            Value::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(f, "if {} {{ {} }}", condition, then_branch)?;
                if let Some(e) = else_branch {
                    write!(f, " else {{ {} }}", e)?;
                }
                Ok(())
            }
        }
    }
}
