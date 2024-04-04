use crate::command_runner::CommandRunner;
use anyhow::Result;

pub fn switch_gcp_env<R: CommandRunner>(runner: &R, project_id: &str) -> Result<()> {
    let zone = "europe-west8";

    runner.run_command(&["config", "set", "project", project_id])?;
    runner.run_command(&[
        "auth",
        "application-default",
        "set-quota-project",
        project_id,
    ])?;

    let cluster_name = if project_id.ends_with("qa") {
        "dataservices-qa"
    } else {
        "dataservices-dev"
    };

    runner.run_command(&[
        "container",
        "clusters",
        "get-credentials",
        cluster_name,
        "--zone",
        zone,
        "--project",
        project_id,
    ])?;

    println!("Switched to {} environment successfully!", project_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_command_runner::MockCommandRunner;

    #[test]
    fn test_switch_gcp_env_success() {
        let runner = MockCommandRunner;
        let project_id = "s-dataservices-qa";
        assert!(switch_gcp_env(&runner, project_id).is_ok());
    }
}
