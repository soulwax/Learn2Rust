//! Thin binary entry point: parse args, call `run`, map any error to a
//! nonzero exit code. Mirrors focus_forge_cli's main.rs shape.

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "focus-forge-status")]
struct Cli {
    #[arg(long, default_value = "site/static/status.json")]
    out: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = focus_forge_status::run(&cli.out) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
