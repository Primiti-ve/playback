use std::error::Error;

use libplayback::workflow::create_workflow;

pub fn new(workflow_name: &str) -> Result<(), Box<dyn Error>> {
    create_workflow(workflow_name)?;

    Ok(())
}
