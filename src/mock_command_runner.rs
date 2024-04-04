use crate::command_runner::CommandRunner;
use anyhow::Result;

pub struct MockCommandRunner;

impl CommandRunner for MockCommandRunner {
    fn run_command(&self, _args: &[&str]) -> Result<()> {
        Ok(())
    }
}
