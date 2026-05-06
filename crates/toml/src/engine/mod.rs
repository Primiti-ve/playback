pub mod context;
pub mod eval;
pub mod exec;

use crate::ast::Value;

use context::Context;
use exec::execute_steps;

pub fn run(root: &std::collections::HashMap<String, Value>, ctx: Context) -> Result<(), String> {
    let workflow = match root.get("workflow") {
        Some(Value::Table(map)) => map,
        _ => return Err("missing workflow".into()),
    };

    let steps = workflow.get("steps").ok_or("missing steps")?;

    execute_steps(steps, &ctx)
}
