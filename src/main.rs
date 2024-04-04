mod command_runner;
mod gcloud_command_runner;
mod mock_command_runner;
mod switch_gcp_env;

use crate::gcloud_command_runner::GCloudCommandRunner;
use crate::switch_gcp_env::switch_gcp_env;
use anyhow::Result;
use std::env;

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

    let runner = GCloudCommandRunner;
    switch_gcp_env(&runner, project_id)?;
    Ok(())
}
