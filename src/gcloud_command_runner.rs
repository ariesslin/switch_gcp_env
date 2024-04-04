use crate::command_runner::CommandRunner;
use anyhow::{anyhow, Result};
use std::process::Command;

pub struct GCloudCommandRunner;

impl CommandRunner for GCloudCommandRunner {
    fn run_command(&self, args: &[&str]) -> Result<()> {
        let output = Command::new("gcloud").args(args).output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!("Command execution failed"))
        }
    }
}
