mod stdin;

use std::{io::Read, process};

use atty::Stream;
use clap::{Parser, Subcommand};
use colored::Colorize;
use fern::Dispatch;
use log::debug;
use stdin::run_stdin;

// use install::run_install;
// use uninstall::run_uninstall;

/// Utility to add ticket id to commit message
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    verbose: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Append ticket id to commit message (must be run on commit)
    Apply {},
    /// Install git hook
    Install {
        /// force installation, overwrite existing hook
        #[arg(short, long)]
        force: bool,
    },
    /// Uninstall git hook
    Uninstall {
        /// force uninstallation, even if not commitfmt hook is installed
        #[arg(short, long)]
        force: bool,
    },
}

fn main() -> process::ExitCode {
    let cli = Cli::parse();

    #[allow(clippy::pedantic)]
    let log_level = match cli.verbose {
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info,
    };
    Dispatch::new().level(log_level).chain(std::io::stdout()).apply().expect("Unable to set up logger");
    if cli.no_color {
        colored::control::set_override(false);
    }

    debug!("Debug: {}", "start".bright_cyan());

    let cwd = std::env::current_dir().unwrap();
    if atty::isnt(Stream::Stdin) {
        let mut buffer = String::with_capacity(1024);
        std::io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");

        return run_stdin(&buffer, &cwd)
    }

    match &cli.command {
        Some(Commands::Apply {}) => {
            unimplemented!();
        }
        Some(Commands::Install {
            force: _,
        }) => unimplemented!(),
        Some(Commands::Uninstall {
            force: _,
        }) => unimplemented!(),
        None => {
            // print!("No command specified");
            process::ExitCode::FAILURE
        }
    }
}
