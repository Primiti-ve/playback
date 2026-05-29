use playback_dsl::{Value, parse};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone)]
pub struct ConfigSchema {
    pub workflows: WorkflowConfig,
    pub exec: ExecConfig,
    pub logging: LoggingConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct WorkflowConfig {
    pub parallel: bool,
    pub shell: String,
    pub max_concurrency: usize,
    pub continue_on_error: bool,
}

#[derive(Debug, Clone)]
pub struct ExecConfig {
    pub timeout: u64,
    pub dry_run: bool,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub dir: String,
}

fn get_table<'a>(map: &'a HashMap<String, Value>, key: &str) -> Result<&'a HashMap<String, Value>, Box<dyn Error>> {
    match map.get(key) {
        Some(Value::Table(t)) => Ok(t),
        Some(_) => Err(format!("'{}' must be a table", key).into()),
        None => Err(format!("missing required table '{}'", key).into()),
    }
}

fn get_bool(map: &HashMap<String, Value>, key: &str, default: bool) -> Result<bool, Box<dyn Error>> {
    match map.get(key) {
        Some(Value::Boolean(v)) => Ok(*v),
        Some(_) => Err(format!("'{}' must be a boolean", key).into()),
        None => Ok(default),
    }
}

fn get_string(map: &HashMap<String, Value>, key: &str, default: &str) -> Result<String, Box<dyn Error>> {
    match map.get(key) {
        Some(Value::String(v)) => Ok(v.clone()),
        Some(_) => Err(format!("'{}' must be a string", key).into()),
        None => Ok(default.to_string()),
    }
}

fn get_integer(map: &HashMap<String, Value>, key: &str, default: i64) -> Result<i64, Box<dyn Error>> {
    match map.get(key) {
        Some(Value::Integer(v)) => Ok(*v),
        Some(_) => Err(format!("'{}' must be an integer", key).into()),
        None => Ok(default),
    }
}

pub fn decode_config(content: &str) -> Result<ConfigSchema, Box<dyn Error>> {
    let parsed = parse(content)?;
    let workflows_tbl = get_table(&parsed, "workflows")?;

    let workflows = WorkflowConfig {
        parallel: get_bool(workflows_tbl, "parallel", false)?,

        shell: get_string(workflows_tbl, "shell", if cfg!(target_os = "windows") { "pwsh" } else { "sh" })?,

        max_concurrency: get_integer(workflows_tbl, "max_concurrency", 4)? as usize,

        continue_on_error: get_bool(workflows_tbl, "continue_on_error", false)?,
    };

    let exec_tbl = get_table(&parsed, "exec")?;
    let exec = ExecConfig {
        timeout: get_integer(exec_tbl, "timeout", 3000)? as u64,

        dry_run: get_bool(exec_tbl, "dry_run", false)?,

        verbose: get_bool(exec_tbl, "verbose", false)?,
    };

    let logging = match parsed.get("logging") {
        Some(Value::Table(tbl)) => LoggingConfig {
            level: get_string(tbl, "level", "info")?,
        },

        Some(_) => return Err("'logging' must be a table".into()),

        None => LoggingConfig { level: "info".to_string() },
    };

    let cache = match parsed.get("cache") {
        Some(Value::Table(tbl)) => CacheConfig {
            enabled: get_bool(tbl, "enabled", true)?,

            dir: get_string(tbl, "dir", ".playback/cache")?,
        },

        Some(_) => return Err("'cache' must be a table".into()),

        None => CacheConfig {
            enabled: true,
            dir: ".playback/cache".to_string(),
        },
    };

    Ok(ConfigSchema { workflows, exec, logging, cache })
}
