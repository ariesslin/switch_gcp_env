use crate::command_runner::CommandRunner;
use anyhow::Result;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref COMMAND_HISTORY: Mutex<Vec<Vec<String>>> = Mutex::new(vec![]);
}

pub struct MockCommandRunner;

impl CommandRunner for MockCommandRunner {
    fn run_command(&self, args: &[&str]) -> Result<()> {
        let mut history = COMMAND_HISTORY.lock().unwrap();
        history.push(args.iter().map(|s| s.to_string()).collect());
        Ok(())
    }
}

#[cfg(test)]
pub fn get_command_history() -> Vec<Vec<String>> {
    COMMAND_HISTORY.lock().unwrap().clone()
}

#[cfg(test)]
pub fn clear_command_history() {
    COMMAND_HISTORY.lock().unwrap().clear();
}
