use clap::{CommandFactory, Parser};

use colored::Colorize;
use std::{io::Read, process};

use commitfmt::ignore::is_ignored_message;
use commitfmt::{
    print_debug, print_error, print_info, print_warning, setup_logger, Commitfmt,
};
use commitfmt_git::Repository;

/// Input source for the commit message.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum InputSource {
    /// The commit message is read from stdin.
    Stdin,
    /// The commit message is read from the commit edit message (e.g. `COMMIT_EDITMSG`).
    CommitEditMessage,
    /// No input source is available.
    None,
}

/// Utility to add ticket id to commit message
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// The lower boundary of the commit range to be checked.
    /// If the `--to` parameter is not set, it will check commits up to the current one
    #[arg(long)]
    pub from: Option<String>,

    /// The upper boundary of the commits range to be checked.
    /// Can be used only together with the `--from` parameter
    #[arg(long)]
    pub to: Option<String>,

    /// Check the message and return an error if any problem is found
    #[arg(short, long)]
    pub lint: bool,
}

/// Returns true if and only if stdin is believed to be readable.
fn is_readable() -> bool {
    #[cfg(unix)]
    fn imp() -> bool {
        use same_file::Handle;
        use std::os::unix::fs::FileTypeExt;

        let ft = match Handle::stdin().and_then(|h| h.as_file().metadata()) {
            Err(_) => return false,
            Ok(md) => md.file_type(),
        };
        ft.is_file() || ft.is_fifo() || ft.is_socket()
    }

    #[cfg(windows)]
    fn imp() -> bool {
        use winapi_util as winutil;

        winutil::file::typ(winutil::HandleRef::stdin())
            .map(|t| t.is_disk() || t.is_pipe())
            .unwrap_or(false)
    }

    !atty::is(atty::Stream::Stdin) && imp()
}

/// Returns the input source for the commit message.
///
/// Tries to determine the input source from the following sources:
/// - stdin
/// - commit edit message
/// - none
fn get_source(repo: &Repository) -> InputSource {
    if is_readable() {
        return InputSource::Stdin;
    }

    if repo.is_committing() {
        return InputSource::CommitEditMessage;
    }

    InputSource::None
}

fn main() -> process::ExitCode {
    let cli = Cli::parse();
    setup_logger(cli.verbose, cli.no_color);

    let Ok(cwd) = std::env::current_dir() else {
        print_error!("Unable to get current directory");
        return process::ExitCode::FAILURE;
    };

    let fmt = match Commitfmt::from_path(&cwd) {
        Ok(fmt) => fmt,
        Err(err) => {
            print_error!("{err}");
            return process::ExitCode::FAILURE;
        }
    };

    if cli.to.is_some() && cli.from.is_none() {
        print_error!("--to requires --from");
        return process::ExitCode::FAILURE;
    }

    if cli.from.is_some() {
        if cli.lint {
            print_warning!("--lint is ignored when --from is set");
        }

        let to = cli.to.as_deref().unwrap_or("HEAD");
        let from = cli.from.as_ref().unwrap();

        if let Err(err) = fmt.lint_commit_range((from, to)) {
            print_error!("{err}");
            return process::ExitCode::FAILURE;
        }
    }

    let source = get_source(&fmt.repo);
    print_debug!("Input source: {source:?}");

    let input = match source {
        InputSource::Stdin => {
            let mut input = String::new();
            if let Err(err) = std::io::stdin().read_to_string(&mut input) {
                print_error!("Failed to read stdin: {err}");
                return process::ExitCode::FAILURE;
            }
            input
        }
        InputSource::CommitEditMessage => match fmt.repo.read_commit_message() {
            Ok(msg) => msg,
            Err(err) => {
                print_error!("Failed to read commit message: {err}");
                return process::ExitCode::FAILURE;
            }
        },
        InputSource::None => {
            print_error!("Unable to determine input source\n");
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
            return process::ExitCode::FAILURE;
        }
    };

    if is_ignored_message(&input) {
        print_warning!("Skipping merge commit");
        return process::ExitCode::SUCCESS;
    }

    let output = match fmt.format_commit_message(&input, cli.lint) {
        Ok(output) => output,
        Err(err) => {
            print_error!("\n{err}");
            return process::ExitCode::FAILURE;
        }
    };
    if cli.lint {
        return process::ExitCode::SUCCESS;
    }

    match source {
        InputSource::Stdin => {
            print_info!("{output}");
        }
        InputSource::CommitEditMessage => {
            if let Err(err) = fmt.repo.write_commit_message(&output) {
                print_error!("Failed to write commit message: {err}");
                return process::ExitCode::FAILURE;
            }
        }
        InputSource::None => {
            unreachable!();
        }
    }

    process::ExitCode::SUCCESS
}
