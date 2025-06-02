use std::env;
use std::error::Error;

use clap::Parser;
use dotenv::dotenv;

mod commands;
mod jira;
use commands::Commands;
use jira::JiraClient;

/// Simple CLI to create Jira issues in a Cloud instance.
///
/// Usage:
///   jira --project <PROJECT> --summary <SUMMARY> --description <DESCRIPTION>
///
/// Example:
///   jira --project PROJ --summary "Login page bug" --description "Fix the authentication issue on the login page"
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args = Args::parse();
    let jira_client = JiraClient::new()?;

    match args.command {
        Commands::Create {
            project,
            summary,
            description,
            issue_type,
        } => {
            jira_client.create_issue(project, summary, description, issue_type)?;
        }
        Commands::LogTime {
            issue_key,
            minutes,
            comment,
        } => {
            jira_client.log_time(issue_key, minutes, comment)?;
        }
    }

    Ok(())
}
