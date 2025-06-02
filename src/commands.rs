use clap::{Subcommand, ValueEnum};
use std::fmt;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum IssueType {
    #[value(aliases = ["bug", "BUG", "Bug"])]
    Bug,
    #[value(aliases = ["task", "TASK", "Task"])]
    Task,
}

impl fmt::Display for IssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueType::Bug => write!(f, "Bug"),
            IssueType::Task => write!(f, "Task"),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Jira issue
    Create {
        /// Jira project key (e.g., "PROJ" for project with key PROJ)
        #[arg(
            long,
            short,
            help = "The key of the Jira project where the issue will be created"
        )]
        project: String,

        /// Issue summary
        #[arg(long, short, help = "The summary of the issue")]
        summary: String,

        /// Issue description
        #[arg(long, short, help = "The detailed description of the issue")]
        description: String,

        /// Type of the issue
        #[arg(
            long("type"),
            short,
            value_enum,
            default_value = "bug",
            help = "Type of the issue (bug or task)"
        )]
        issue_type: IssueType,
    },

    /// Log time spent on a Jira issue
    LogTime {
        /// Jira issue key (e.g., "PROJ-123")
        #[arg(long, short, help = "The key of the Jira issue to log time for")]
        issue_key: String,

        /// Time spent in minutes
        #[arg(long, short, help = "Time spent in minutes")]
        minutes: u32,

        /// Optional comment about the work done
        #[arg(long, short, help = "Optional comment about the work done")]
        comment: Option<String>,
    },
}
