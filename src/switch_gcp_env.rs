use crate::command_runner::CommandRunner;
use anyhow::Result;

pub fn switch_gcp_env<R: CommandRunner>(
    runner: &R,
    project_id: &str,
    cluster_name: Option<&str>,
    zone: Option<&str>,
) -> Result<()> {
    let default_zone = "europe-west8";
    // Default cluster_name for internal workaround, modify as necessary:)
    let default_cluster_name = if project_id.ends_with("qa") {
        "dataservices-qa"
    } else {
        "dataservices-dev"
    };

    let zone = zone.unwrap_or(default_zone);
    let cluster_name = cluster_name.unwrap_or(default_cluster_name);

    runner.run_command(&["config", "set", "project", project_id])?;
    runner.run_command(&[
        "auth",
        "application-default",
        "set-quota-project",
        project_id,
    ])?;

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
    fn test_switch_gcp_env_generic_project_id() {
        clear_command_history();

        let runner = MockCommandRunner;
        let project_id = "custom-gcp-project-id";
        switch_gcp_env(&runner, project_id, None, None)
            .expect("Expected switch_gcp_env to succeed");

        let history = get_command_history();
        assert_eq!(history.len(), 3, "Expected three commands to be run");

        assert_eq!(
            history[0],
            vec![
                "config".to_string(),
                "set".to_string(),
                "project".to_string(),
                project_id.to_string()
            ]
        );
        assert_eq!(
            history[1],
            vec![
                "auth".to_string(),
                "application-default".to_string(),
                "set-quota-project".to_string(),
                project_id.to_string()
            ]
        );

        let command_vector = &history[2];
        assert!(command_vector.contains(&"container".to_string()));
        assert!(command_vector.contains(&"clusters".to_string()));
        assert!(command_vector.contains(&"get-credentials".to_string()));
        assert!(command_vector.contains(&project_id.to_string()));
    }

    #[test]
    fn test_switch_gcp_env_with_cluster_name_and_zone() {
        clear_command_history();

        let runner = MockCommandRunner;
        let project_id = "custom-gcp-project-id";
        let cluster_name = "custom-cluster";
        let zone = "custom-zone";
        switch_gcp_env(&runner, project_id, Some(cluster_name), Some(zone))
            .expect("Expected switch_gcp_env to succeed with custom cluster and zone");

        let history = get_command_history();
        assert_eq!(history.len(), 3, "Expected three commands to be run");

        assert_eq!(
            history[0],
            vec![
                "config".to_string(),
                "set".to_string(),
                "project".to_string(),
                project_id.to_string()
            ]
        );
        assert_eq!(
            history[1],
            vec![
                "auth".to_string(),
                "application-default".to_string(),
                "set-quota-project".to_string(),
                project_id.to_string()
            ]
        );

        assert!(history.contains(&vec![
            "container".to_string(),
            "clusters".to_string(),
            "get-credentials".to_string(),
            cluster_name.to_string(),
            "--zone".to_string(),
            zone.to_string(),
            "--project".to_string(),
            project_id.to_string(),
        ]));
    }
}
