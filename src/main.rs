use std::process::ExitCode;

use clap::Parser;

use crate::command::Command;

mod aws_regional_product_services;
mod cli;
mod command;
mod config;

mod output;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    config::init()?;
    let cli = cli::Cli::parse();
    let retriever = aws_regional_product_services::Retriever::new()?;

    cli.subcommand.execute(&retriever).await
}
