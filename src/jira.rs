use std::error::Error;
use reqwest::blocking::Client;
use serde_json::json;
use chrono;

use crate::commands::{Commands, IssueType};

/// A client for interacting with the Jira API.
pub struct JiraClient {
    domain: String,
    token: String,
    username: String,
    client: Client,
}

impl JiraClient {
    /// Creates a new JiraClient using environment variables.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let domain = get_env_var("JIRA_DOMAIN")?;
        let token = get_env_var("JIRA_API_TOKEN")?;
        let username = get_env_var("JIRA_USER")?;
        let client = Client::new();
        Ok(JiraClient {
            domain,
            token,
            username,
            client,
        })
    }

    /// Creates a new Jira issue with the given project, summary, description, and issue type.
    pub fn create_issue(&self, project: String, summary: String, description: String, issue_type: IssueType) -> Result<(), Box<dyn Error>> {
        let url = format!("https://{}/rest/api/3/issue", self.domain);

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

        println!(
            "Creating a new {} in Jira: {}",
            format!("{}", issue_type).to_lowercase(),
            summary
        );

        let resp = self.client
            .post(&url)
            .basic_auth(&self.username, Some(&self.token))
            .json(&body)
            .send()?;

        let status = resp.status();
        if !status.is_success() {
            let error_text = resp.text()?;
            return Err(format!(
                "Failed to create issue (status: {}): {}",
                status, error_text
            )
            .into());
        }

        let resp_json: serde_json::Value = resp.json()?;
        let key = resp_json
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or("Response missing 'key' field")?;

        println!("Created issue: https://{}/browse/{}", self.domain, key);
        println!("Key: {}", key);
        println!("Summary: {}", summary);
        Ok(())
    }

    /// Logs time spent on a Jira issue with the given issue key, minutes, and optional comment.
    pub fn log_time(&self, issue_key: String, minutes: u32, comment: Option<String>) -> Result<(), Box<dyn Error>> {
        let url = format!("https://{}/rest/api/3/issue/{}/worklog", self.domain, issue_key);

        let body = json!({
            "timeSpentSeconds": minutes * 60,
            "comment": {
                "type": "doc",
                "version": 1,
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": comment.unwrap_or_default()
                            }
                        ]
                    }
                ]
            },
            "started": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S.%3f%z").to_string()
        });

        println!("Logging {} minutes for issue {}", minutes, issue_key);

        let resp = self.client
            .post(&url)
            .basic_auth(&self.username, Some(&self.token))
            .json(&body)
            .send()?;

        let status = resp.status();
        if !status.is_success() {
            let error_text = resp.text()?;
            return Err(
                format!("Failed to log time (status: {}): {}", status, error_text).into(),
            );
        }

        println!("Successfully logged time for issue {}", issue_key);
        Ok(())
    }
}

// Helper function to get an environment variable
fn get_env_var(key: &str) -> Result<String, Box<dyn Error>> {
    std::env::var(key).map_err(|_| format!("Environment variable {} must be set", key).into())
} 