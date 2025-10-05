use clap::{Parser, Subcommand};
use crate::models::Blockchain;
use crate::storage::Storage;

pub struct Cli {
    pub command: Commands,
}

pub enum 