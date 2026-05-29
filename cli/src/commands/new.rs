use playback_lib::workflow::create_workflow;
use std::error::Error;

pub fn new(workflow_name: &str) -> Result<(), Box<dyn Error>> {
    create_workflow(workflow_name)?;

    Ok(())
}
