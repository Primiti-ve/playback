use playback_lib::workflow::run_workflow;
use std::error::Error;

pub fn run(workflow_name: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    run_workflow(workflow_name, args)?;

    Ok(())
}
