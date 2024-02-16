use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Info,
    Init {
        #[arg(value_name = "CONTEST")]
        contest_name: String,
        #[arg(value_name = "TASK")]
        task_name: String,
        #[arg(value_name = "PROFILE")]
        profile_name: String,
        #[arg(value_name = "DIRECTORY")]
        path: Option<String>,
    },
}
