use super::context::Context;
use crate::ast::{Expr, Value};

pub fn eval_expr(expr: &Expr, ctx: &Context) -> Result<bool, String> {
    match expr {
        Expr::Equals(lhs, rhs) => Ok(eval_atom(lhs, ctx)? == eval_atom(rhs, ctx)?),
        Expr::NotEquals(lhs, rhs) => Ok(eval_atom(lhs, ctx)? != eval_atom(rhs, ctx)?),
        _ => Err("unsupported expression".into()),
    }
}

fn eval_atom(expr: &Expr, ctx: &Context) -> Result<String, String> {
    match expr {
        Expr::Identifier(name) => ctx.get(name).cloned().ok_or_else(|| format!("unknown variable: {}", name)),
        Expr::String(s) => Ok(s.clone()),
        _ => Err("invalid atom".into()),
    }
}

pub fn eval_value(value: &Value, ctx: &Context) -> Result<Value, String> {
    match value {
        Value::Conditional { condition, then_branch, else_branch } => {
            if eval_expr(condition, ctx)? {
                eval_value(then_branch, ctx)
            } else if let Some(else_b) = else_branch {
                eval_value(else_b, ctx)
            } else {
                Err("no else branch".into())
            }
        }

        Value::Array(arr) => {
            let mut result = Vec::new();
            for v in arr {
                result.push(eval_value(v, ctx)?);
            }
            Ok(Value::Array(result))
        }

        Value::Table(map) => {
            let mut result = std::collections::HashMap::new();
            for (k, v) in map {
                result.insert(k.clone(), eval_value(v, ctx)?);
            }
            Ok(Value::Table(result))
        }

        _ => Ok(value.clone()),
    }
}
