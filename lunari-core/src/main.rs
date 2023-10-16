use anyhow;
use clap::Parser;
mod phase;
mod cli;

use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lunar(lunar) => {
            println!("{:?}, {:?}", lunar.action, phase::Phase::new(&lunar.date.clone().ok_or("Pija").unwrap()))
        }
    };
    Ok(())
}
