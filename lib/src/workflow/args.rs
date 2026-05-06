use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

use crate::schemas::{Arguments, WorkflowSchema};

pub fn run_steps(
    manifest: &WorkflowSchema,
    ctx: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    for step in &manifest.steps {
        println!("\n> step: {}", step.name);
        println!("  command: {}", step.command);

        let args = resolve_arguments(&step.arguments, ctx)?;

        println!("  args: {:?}", args);

        run_command(manifest, &step.command, &args)?;
    }

    Ok(())
}

pub fn resolve_arguments(
    args: &Arguments,
    ctx: &HashMap<String, String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    match args {
        Arguments::List(list) => Ok(list.iter().map(|s| interpolate(s, ctx)).collect()),

        Arguments::Conditional {
            condition,
            then_args,
            else_args,
        } => {
            if eval_condition(condition, ctx) {
                Ok(then_args.iter().map(|s| interpolate(s, ctx)).collect())
            } else if let Some(else_args) = else_args {
                Ok(else_args.iter().map(|s| interpolate(s, ctx)).collect())
            } else {
                Ok(vec![])
            }
        }
    }
}

pub fn interpolate(input: &str, ctx: &HashMap<String, String>) -> String {
    let mut out = input.to_string();

    for (k, v) in ctx {
        let pattern = format!("{{{{{}}}}}", k);
        out = out.replace(&pattern, v);
    }

    out
}

pub fn eval_condition(expr: &playback_toml::Expr, ctx: &HashMap<String, String>) -> bool {
    use playback_toml::Expr::*;

    match expr {
        Identifier(name) => ctx.get(name).is_some(),
        String(_) => true,
        NotEquals(lhs, rhs) => eval_expr(lhs, ctx) != eval_expr(rhs, ctx),
        Equals(lhs, rhs) => eval_expr(lhs, ctx) == eval_expr(rhs, ctx),
    }
}

pub fn eval_expr(expr: &playback_toml::Expr, ctx: &HashMap<String, String>) -> String {
    use playback_toml::Expr::*;

    match expr {
        Identifier(name) => ctx.get(name).cloned().unwrap_or_default(),
        String(s) => s.clone(),
        _ => "".to_string(),
    }
}

pub fn quote_arg(arg: &str) -> String {
    let escaped = arg.replace('"', r#""""#);

    format!("\"{}\"", escaped)
}

pub fn run_command(
    manifest: &WorkflowSchema,
    command: &str,
    args: &[String],
) -> Result<(), Box<dyn Error>> {
    let shell = manifest
        .env
        .as_ref()
        .and_then(|e| e.shell.clone())
        .unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "pwsh".to_string()
            } else {
                "sh".to_string()
            }
        });

    // Build command string
    let mut full_cmd = String::new();
    full_cmd.push_str(command);

    for arg in args {
        full_cmd.push(' ');
        full_cmd.push_str(&quote_arg(arg));
    }

    let mut cmd = if cfg!(target_os = "windows") {
        match shell.as_str() {
            "pwsh" | "powershell" => {
                let mut c = Command::new("pwsh");
                c.arg("-NoProfile").arg("-Command").arg(&full_cmd);
                c
            }
            "cmd" => {
                let mut c = Command::new("cmd");
                c.arg("/C").arg(&full_cmd);
                c
            }
            _ => {
                // fallback (rare)
                let mut c = Command::new(command);
                c.args(args);
                c
            }
        }
    } else {
        let mut c = Command::new(shell);
        c.arg("-c").arg(&full_cmd);
        c
    };

    // ✅ Apply cwd + env (THIS WAS MISSING)
    if let Some(env) = &manifest.env {
        if let Some(cwd) = &env.cwd {
            cmd.current_dir(cwd);
        }

        for (k, v) in &env.vars {
            cmd.env(k, v);
        }
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err(format!("command failed: {}", full_cmd).into());
    }

    Ok(())
}
