mod runner;
mod build;
mod init;

use clap::{Parser, Subcommand};
use fabela_core::error::Result;
use std::{path::PathBuf, process};

use crate::{build::build_file, init::init_tracing, runner::{run_embedded, run_file}};

#[derive(Parser)]
#[command(
  name = "fabela",
  version,
  about = "🥀 tiny js runtime ;;"
)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
  Run {
    file: PathBuf,
  },
  Build {
    file: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
  }
}

fn main() {
    init_tracing();

    if let Err(e) = try_run() {
        eprintln!("{:?}", miette::Report::new(e));
        process::exit(1);
    }
}

fn try_run() -> Result<()> {
    if run_embedded()? {
        return Ok(());
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { file }) => run_file(file)?,
        Some(Commands::Build { file, output }) => build_file(file, output)?,
        None => print_help(),
    }

    Ok(())
}

fn print_help() {
    eprintln!("Fabela — Tiny JavaScript runtime\n");
    eprintln!("Usage:");
    eprintln!("  fabela run <file.js>        Run a JavaScript file");
    eprintln!("  fabela build <file.js>      Compile to standalone executable");
    eprintln!("  fabela --version            Show version");
    eprintln!("  fabela --help               Show help");
}
