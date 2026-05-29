use playback_dsl::{Expr, Value, parse};
use std::{collections::HashMap, error::Error};

#[derive(Debug)]
pub struct WorkflowSchema {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub env: Option<EnvConfig>,
    pub steps: Vec<Step>,
}

#[derive(Debug)]
pub struct EnvConfig {
    pub shell: Option<String>,
    pub cwd: Option<String>,
    pub vars: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Step {
    pub name: String,
    pub command: String,
    pub arguments: Arguments,
    pub order: usize,
}

#[derive(Debug)]
pub enum Arguments {
    List(Vec<String>),
    Conditional {
        condition: Expr,
        then_args: Vec<String>,
        else_args: Option<Vec<String>>,
    },
}

fn get_string(map: &HashMap<String, Value>, key: &str) -> Result<String, Box<dyn Error>> {
    match map.get(key) {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(_) => Err(format!("expected '{}' to be a string", key).into()),
        None => Err(format!("missing required key '{}'", key).into()),
    }
}

fn get_optional_string(map: &HashMap<String, Value>, key: &str) -> Option<String> {
    match map.get(key) {
        Some(Value::String(s)) => Some(s.clone()),
        _ => None,
    }
}

fn get_table<'a>(map: &'a HashMap<String, Value>, key: &str) -> Result<&'a HashMap<String, Value>, Box<dyn Error>> {
    match map.get(key) {
        Some(Value::Table(t)) => Ok(t),
        Some(_) => Err(format!("expected '{}' to be a table", key).into()),
        None => Err(format!("missing required table '{}'", key).into()),
    }
}

fn parse_env(workflow: &HashMap<String, Value>) -> Result<Option<EnvConfig>, Box<dyn Error>> {
    let env_tbl = match workflow.get("env") {
        Some(Value::Table(t)) => t,
        None => return Ok(None),
        _ => return Err("workflow.env must be a table".into()),
    };

    let shell = get_optional_string(env_tbl, "shell");
    let cwd = get_optional_string(env_tbl, "cwd");

    let mut vars = HashMap::new();

    if let Some(Value::Table(env_map)) = env_tbl.get("env") {
        for (k, v) in env_map {
            match v {
                Value::String(s) => {
                    vars.insert(k.clone(), s.clone());
                }
                _ => return Err(format!("env var '{}' must be string", k).into()),
            }
        }
    }

    Ok(Some(EnvConfig { shell, cwd, vars }))
}

fn parse_steps(workflow: &HashMap<String, Value>) -> Result<Vec<Step>, Box<dyn Error>> {
    let steps_tbl = get_table(workflow, "steps")?;

    let mut steps = Vec::new();

    for (name, value) in steps_tbl {
        let step_tbl = match value {
            Value::Table(t) => t,

            _ => return Err(format!("step '{}' must be a table", name).into()),
        };

        let command = get_string(step_tbl, "command")?;
        let arguments = parse_arguments(step_tbl)?;

        let order = match step_tbl.get("order") {
            Some(Value::Integer(i)) if *i >= 0 => *i as usize,

            Some(_) => {
                return Err(format!("step '{}' order must be a non-negative integer", name).into());
            }

            None => return Err(format!("step '{}' is missing required field 'order'", name).into()),
        };

        steps.push(Step {
            name: name.clone(),
            command,
            arguments,
            order,
        });
    }

    steps.sort_by_key(|s| s.order);

    Ok(steps)
}

fn parse_arguments(step: &HashMap<String, Value>) -> Result<Arguments, Box<dyn Error>> {
    let value = step.get("arguments").ok_or("missing 'arguments'")?;

    match value {
        Value::Array(arr) => {
            let mut args = Vec::new();

            for v in arr {
                match v {
                    Value::String(s) => args.push(s.clone()),
                    _ => return Err("arguments must be strings".into()),
                }
            }

            Ok(Arguments::List(args))
        }

        Value::Conditional { condition, then_branch, else_branch } => {
            let then_args = extract_array(then_branch)?;
            let else_args = match else_branch {
                Some(v) => Some(extract_array(v)?),
                None => None,
            };

            Ok(Arguments::Conditional {
                condition: condition.clone(),
                then_args,
                else_args,
            })
        }

        _ => Err("invalid arguments type".into()),
    }
}

fn extract_array(value: &Value) -> Result<Vec<String>, Box<dyn Error>> {
    match value {
        Value::Array(arr) => {
            let mut result = Vec::new();

            for v in arr {
                match v {
                    Value::String(s) => result.push(s.clone()),
                    _ => return Err("array must contain only strings".into()),
                }
            }

            Ok(result)
        }
        _ => Err("expected array".into()),
    }
}

pub fn decode_workflow(content: &str) -> Result<WorkflowSchema, Box<dyn Error>> {
    let parsed = parse(content)?;

    let name = get_string(&parsed, "name")?;
    let description = get_string(&parsed, "description")?;
    let version = get_string(&parsed, "version")?;
    let author = get_optional_string(&parsed, "author");

    let workflow = get_table(&parsed, "workflow")?;

    let env = parse_env(workflow)?;
    let steps = parse_steps(workflow)?;

    Ok(WorkflowSchema {
        name,
        description,
        version,
        author,
        env,
        steps,
    })
}
