use std::error::Error;

use libplayback::workflow::run_workflow;

pub fn run(workflow_name: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    run_workflow(workflow_name, args)?;

    Ok(())
}
