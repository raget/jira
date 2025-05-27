use std::env;
use std::error::Error;

use clap::{Parser, Subcommand, ValueEnum};
use reqwest::blocking::Client;
use serde_json::json;
use dotenv::dotenv;

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

#[derive(ValueEnum, Clone, Debug)]
#[derive(PartialEq)]
enum IssueType {
    #[value(aliases = ["bug", "BUG", "Bug"])]
    Bug,
    #[value(aliases = ["task", "TASK", "Task"])]
    Task,
}

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueType::Bug => write!(f, "Bug"),
            IssueType::Task => write!(f, "Task"),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Jira issue
    Create {
        /// Jira project key (e.g., "PROJ" for project with key PROJ)
        #[arg(long, short, help = "The key of the Jira project where the issue will be created")]
        project: String,

        /// Issue summary
        #[arg(long, short, help = "The summary of the issue")]
        summary: String,

        /// Issue description
        #[arg(long, short, help = "The detailed description of the issue")]
        description: String,

        /// Type of the issue
        #[arg(long("type"), short, value_enum, default_value = "bug", help = "Type of the issue (bug or task)")]
        issue_type: IssueType,
    },
}

fn get_env_var(key: &str) -> Result<String, Box<dyn Error>> {
    env::var(key).map_err(|_| format!("Environment variable {} must be set", key).into())
}

fn main() -> Result<(), Box<dyn Error>> {    
    dotenv().ok();

    let args = Args::parse();

    match args.command {
        Commands::Create { project, summary, description, issue_type } => {
            let domain = get_env_var("JIRA_DOMAIN")?;
            let token = get_env_var("JIRA_API_TOKEN")?;
            let username = get_env_var("JIRA_USER")?;

            let client = Client::new();
            let url = format!("https://{}/rest/api/3/issue", domain);

            let body = json!({
                "fields": {
                    "project": { "key": project },
                    "summary": summary,
                    "description": {
                        "type": "doc",
                        "version": 1,
                        "content": [
                            {
                                "type": "paragraph",
                                "content": [
                                    {
                                        "type": "text",
                                        "text": description
                                    }
                                ]
                            }
                        ]
                    },
                    "issuetype": { "name": format!("{}", issue_type) },
                    "priority": { "name": "Medium priority (C)" } 
                }
            });

            println!("Creating a new {} in Jira: {}", format!("{}", issue_type).to_lowercase(), summary);

            let resp = client
                .post(&url)
                .basic_auth(username, Some(token))
                .json(&body)
                .send()?;

            let status = resp.status();
            if !status.is_success() {
                let error_text = resp.text()?;
                return Err(format!(
                    "Failed to create issue (status: {}): {}",
                    status,
                    error_text
                ).into());
            }

            let resp_json: serde_json::Value = resp.json()?;
            let key = resp_json
                .get("key")
                .and_then(|v| v.as_str())
                .ok_or("Response missing 'key' field")?;

            println!("Created issue: https://{}/browse/{}", domain, key);
            println!("Key: {}", key);
            println!("Summary: {}", summary);
        }
    }

    Ok(())
}
