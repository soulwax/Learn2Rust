//! `focus_forge_cli` is the command-line front end for Focus Forge. It owns
//! argument parsing and terminal output only; every domain rule lives in
//! `focus_forge_core`. Compare a thin controller layer in an MVC app — this
//! crate translates between the shell and the domain, and nothing more.

pub mod commands;
pub mod output;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use focus_forge_core::CoreError;
use thiserror::Error;

/// Top-level CLI: a global `--file` option plus one subcommand tree. `derive
/// (Parser)` is clap's macro-generated argument parser — the same role a
/// `[Verb]`/decorator-driven CLI framework plays in C#/Java, but the parser
/// itself is generated from this struct's shape at compile time.
#[derive(Debug, Parser)]
#[command(name = "focus-forge")]
pub struct Cli {
    /// Path to the JSON workspace file. Defaults to `workspace.json` in the
    /// current directory.
    #[arg(long, default_value = "workspace.json")]
    pub file: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create, list, and show projects.
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },
    /// Add tasks to a project and mark them done.
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
    /// Add notes to a project.
    Note {
        #[command(subcommand)]
        action: NoteAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum ProjectAction {
    /// Add a new project. Bootstraps the workspace file if it does not exist.
    Add { id: String, name: String },
    /// List every project in the workspace.
    List,
    /// Show one project's tasks and notes.
    Show { id: String },
}

#[derive(Debug, Subcommand)]
pub enum TaskAction {
    /// Add a task to an existing project.
    Add {
        project_id: String,
        task_id: String,
        title: String,
        /// One of `low`, `medium`, `high`. Kept as a plain string here (not
        /// `focus_forge_core::Priority` directly) because `focus_forge_core`
        /// must not depend on clap; `commands.rs` parses it into the real
        /// domain type. Defaults to medium, matching `Priority`'s
        /// `#[default]` variant.
        #[arg(long, default_value = "medium")]
        priority: String,
    },
    /// Mark a task done.
    Done { project_id: String, task_id: String },
}

#[derive(Debug, Subcommand)]
pub enum NoteAction {
    /// Add a note to an existing project.
    Add {
        project_id: String,
        note_id: String,
        text: String,
    },
}

/// Every error `run` can return. `#[from]` on `Core` lets `?` convert a
/// `CoreError` automatically, the same way a C# `catch` block can rethrow a
/// lower-level exception wrapped in a domain-specific one.
#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error(
        "workspace file not found: {0} (create a project first with: project add <id> <name>)"
    )]
    WorkspaceNotFound(String),
    #[error("invalid priority: {0} (expected one of: low, medium, high)")]
    InvalidPriority(String),
    #[error("unknown task id: {0}")]
    UnknownTask(String),
}

/// Runs one parsed CLI invocation: dispatches to a `commands` handler, then
/// formats and prints via `output`. This is the crate's single fallible
/// entry point — `main.rs` only maps its `Err` to an exit code, the same
/// role a top-level `try`/`catch` around `Main` plays in C#.
pub fn run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Command::Project { action } => match action {
            ProjectAction::Add { id, name } => commands::project_add(&cli.file, &id, &name)?,
            ProjectAction::List => {
                let workspace = commands::project_list(&cli.file)?;
                println!("{}", output::format_project_list(&workspace));
            }
            ProjectAction::Show { id } => {
                let project = commands::project_show(&cli.file, &id)?;
                println!("{}", output::format_project_show(&project));
            }
        },
        Command::Task { action } => match action {
            TaskAction::Add {
                project_id,
                task_id,
                title,
                priority,
            } => commands::task_add(&cli.file, &project_id, &task_id, &title, &priority)?,
            TaskAction::Done {
                project_id,
                task_id,
            } => commands::task_done(&cli.file, &project_id, &task_id)?,
        },
        Command::Note { action } => match action {
            NoteAction::Add {
                project_id,
                note_id,
                text,
            } => commands::note_add(&cli.file, &project_id, &note_id, &text)?,
        },
    }
    Ok(())
}
