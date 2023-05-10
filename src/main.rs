use std::process::ExitCode;
use cli::Cli;
use clap::Parser;

mod cli;

fn main() -> ExitCode {
    let args = Cli::parse();
    args.run();
    return ExitCode::SUCCESS;
}