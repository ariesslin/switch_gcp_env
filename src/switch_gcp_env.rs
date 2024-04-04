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
    use crate::mock_command_runner::{
        clear_command_history, get_command_history, MockCommandRunner,
    };

    #[test]
    fn test_switch_gcp_env_success() {
        clear_command_history();

        let runner = MockCommandRunner;
        let project_id = "s-dataservices-qa";
        switch_gcp_env(&runner, project_id).expect("Expected switch_gcp_env to succeed");

        let history = get_command_history();
        assert_eq!(history.len(), 3, "Expected three commands to be run");

        assert_eq!(
            history[0],
            vec!["config", "set", "project", "s-dataservices-qa"]
        );
        assert_eq!(
            history[1],
            vec![
                "auth",
                "application-default",
                "set-quota-project",
                "s-dataservices-qa"
            ]
        );
        assert!(history[2].contains(&"container".to_string()));
        assert!(history[2].contains(&"clusters".to_string()));
        assert!(history[2].contains(&"get-credentials".to_string()));
        assert!(history[2].contains(&"dataservices-qa".to_string()));
    }
}
