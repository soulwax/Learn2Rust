//! Thin binary entry point: parse args, call `run`, map any error to a
//! nonzero exit code. All real logic lives in the library (`lib.rs`,
//! `commands.rs`, `output.rs`) so it can be unit-tested directly.

use clap::Parser;
use focus_forge_cli::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = focus_forge_cli::run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
