use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="blockjournal")]
#[command(about="A journal blockchain CLI application")]
#[command(version="1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {

}