mod ast;
mod cli;
mod codegen;
mod ir;
mod parser;
mod runtime;

use clap::Parser as ClapParser;
use cli::{Cli, Commands};
use colored::*;
use std::process::ExitCode;

// Main entry point for the E++ CLI
fn main() -> ExitCode {
    let cli_args = Cli::parse();
    let result = match cli_args.command {
        Commands::New { project_name } => cli::handle_new_project(&project_name),
        Commands::Build { file, output } => cli::handle_build(&file, output.as_deref()),
        Commands::Run { file } => cli::handle_run(&file),
        Commands::Install { package } => cli::handle_install(&package),
        Commands::Test => cli::handle_test(),
    };
    match result {
        Ok(message) => {
            if !message.is_empty() {
                println!("{}", message.green());
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}
