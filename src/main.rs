mod command_runner;
mod gcloud_command_runner;
mod mock_command_runner;
mod switch_gcp_env;

use crate::gcloud_command_runner::GCloudCommandRunner;
use crate::switch_gcp_env::switch_gcp_env;
use anyhow::Result;
use clap::{Arg, ArgAction, Command}; // Note: It's `Command` in clap v3, not `App`

fn main() -> Result<()> {
    let matches = Command::new("switch_gcp_env")
        .version("0.0.1")
        .about("Switches GCP environment configurations.")
        .arg(
            Arg::new("project_id")
                .help("The GCP project ID to switch to")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("cluster_name")
                .long("cluster-name")
                .action(ArgAction::Set)
                .value_name("CLUSTER_NAME")
                .help("Optional: The name of the GCP Kubernetes cluster"),
        )
        .arg(
            Arg::new("zone")
                .long("zone")
                .action(ArgAction::Set)
                .value_name("ZONE")
                .help("Optional: The GCP zone of the Kubernetes cluster")
                .default_value("europe-west8"),
        )
        .get_matches();

    let project_id = matches.get_one::<String>("project_id").unwrap();
    let cluster_name = matches.get_one::<String>("cluster_name");
    let zone = matches.get_one::<String>("zone").unwrap();

    let runner = GCloudCommandRunner;
    switch_gcp_env(
        &runner,
        project_id,
        cluster_name.map(String::as_str),
        Some(zone),
    )?;
    Ok(())
}
