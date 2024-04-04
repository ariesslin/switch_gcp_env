use anyhow::Result;

pub trait CommandRunner {
    fn run_command(&self, args: &[&str]) -> Result<()>;
}
