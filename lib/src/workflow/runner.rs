use std::collections::HashMap;
use std::error::Error;

use crate::schemas::decode_workflow;

use super::args::run_steps;

pub fn run_workflow(workflow_name: &str, cli_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    log::info!("running workflow `{}`", workflow_name);

    let content = std::fs::read_to_string(format!(".playback/workflows/{}.toml", workflow_name))?;
    let manifest = decode_workflow(content.as_str())?;

    log::debug!("manifest: {:#?}", manifest);

    let mut ctx = HashMap::new();

    if let Some(env) = &manifest.env {
        for (k, v) in &env.vars {
            ctx.insert(k.clone(), v.clone());
        }
    }

    for (i, arg) in cli_args.iter().enumerate() {
        if let Some((key, value)) = arg.split_once('=') {
            ctx.insert(key.to_string(), value.to_string());
        } else {
            match i {
                0 => ctx.insert("title".into(), arg.clone()),
                1 => ctx.insert("description".into(), arg.clone()),

                _ => None,
            };
        }
    }

    run_steps(&manifest, &ctx)?;

    Ok(())
}
