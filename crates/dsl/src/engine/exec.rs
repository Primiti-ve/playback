use std::process::Command;

use super::{context::Context, eval::eval_value};
use crate::ast::Value;

pub fn execute_steps(steps: &Value, ctx: &Context) -> Result<(), String> {
    let steps_table = match steps {
        Value::Table(map) => map,
        _ => return Err("steps must be a table".into()),
    };

    for step in steps_table.values() {
        execute_step(step, ctx)?;
    }

    Ok(())
}

fn execute_step(step: &Value, ctx: &Context) -> Result<(), String> {
    let table = match step {
        Value::Table(map) => map,
        _ => return Err("step must be a table".into()),
    };

    let command = match table.get("command") {
        Some(Value::String(s)) => s.clone(),

        _ => return Err("missing command".into()),
    };

    let args_value = table.get("arguments");

    let mut args: Vec<String> = vec![];

    if let Some(val) = args_value {
        let evaluated = eval_value(val, ctx)?;

        match evaluated {
            Value::Array(arr) => {
                for v in arr {
                    match v {
                        Value::String(s) => args.push(interpolate(&s, ctx)),

                        _ => return Err("arguments must be strings".into()),
                    }
                }
            }
            _ => return Err("arguments must be array".into()),
        }
    }

    println!("  {} {:?}", command, args);

    let status = Command::new(&command).args(&args).status().map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!("command failed: {}", command));
    }

    Ok(())
}

fn interpolate(input: &str, ctx: &Context) -> String {
    let mut result = input.to_string();

    for (key, val) in &ctx.vars {
        let pattern = format!("{{{{{}}}}}", key);

        result = result.replace(&pattern, val);
    }

    result
}
