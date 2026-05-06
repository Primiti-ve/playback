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

                if let Some(else_b) = else_branch {
                    write!(f, " else {{ {} }}", else_b)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "parse error at line {}, column {}: {}\nline:\n{:?}",
            self.line, self.column, self.message, self.snippet
        )
    }
}

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Parser<'a> {
    

    

    

    
}

pub fn parse(input: &str) -> Result<HashMap<String, Value>, ParseError> {
    Parser::new(input).parse()
}
