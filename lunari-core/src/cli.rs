use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum Commands {
    /// Moon related commands
    Lunar (Lunar),
    // Sun related commands
    // Solar (Solar),
}

#[derive(Clone, Debug, ValueEnum)]
pub enum LunarAction {
    Phase,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Lunar {
    pub action: LunarAction,
    #[arg(short, long)]
    pub date: Option<String>
}
