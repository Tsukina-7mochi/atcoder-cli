use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(long)]
    pub env_session: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Info,
    Init {
        #[arg(value_name = "PROFILE")]
        profile_name: String,
        #[arg(value_name = "CONTEST")]
        contest_name: String,
        #[arg(value_name = "TASK")]
        task_name: Option<String>,
        #[arg(value_name = "DIRECTORY")]
        path: Option<String>,
    },
    Url {
        #[arg(value_name = "CONTEST")]
        contest_name: Option<String>,
    },
    Run {
        #[arg(long)]
        manual: bool,
    },
    Test {
        #[arg(long)]
        manual: bool,
    },
    Login,
    Submit,
}
