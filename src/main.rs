use anyhow::Result;
use std::env;
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: switch_gcp_env <environment>");
        std::process::exit(1);
    }

    let env = &args[1];
    let project_id = match env.as_str() {
        "qa" => "s-dataservices-qa",
        "dev" => "s-dataservices",
        _ => {
            eprintln!("Invalid environment specified");
            std::process::exit(1);
        }
    };

    switch_gcp_env(project_id)?;
    Ok(())
}

fn switch_gcp_env(project_id: &str) -> Result<()> {
    let zone = "europe-west8";

    let set_project = Command::new("gcloud")
        .args(["config", "set", "project", project_id])
        .output()?;

    let set_quota_project = Command::new("gcloud")
        .args([
            "auth",
            "application-default",
            "set-quota-project",
            project_id,
        ])
        .output()?;

    let cluster_name = if project_id.ends_with("qa") {
        "dataservices-qa"
    } else {
        "dataservices-dev"
    };

    let get_credentials = Command::new("gcloud")
        .args([
            "container",
            "clusters",
            "get-credentials",
            cluster_name,
            "--zone",
            zone,
            "--project",
            project_id,
        ])
        .output()?;

    if set_project.status.success()
        && set_quota_project.status.success()
        && get_credentials.status.success()
    {
        println!("Switched to {} environment successfully!", project_id);
    } else {
        eprintln!("Failed to switch to the {} environment.", project_id);
        std::process::exit(1);
    }

    Ok(())
}
