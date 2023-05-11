use std::process::ExitCode;
use cli::Cli;
use clap::Parser;

mod cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    return if cli.run() {ExitCode::SUCCESS} else {ExitCode::FAILURE};
}