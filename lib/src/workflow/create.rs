use playback_logger::info;
use std::error::Error;

pub fn create_workflow(workflow_name: &str) -> Result<(), Box<dyn Error>> {
    info!("lib::workflow::create", "creating workflow `{}`", workflow_name);

    Ok(())
}
