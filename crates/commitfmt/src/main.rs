mod cli;
mod logging;
mod report;
mod stdin;

use clap::{CommandFactory, Parser};
use cli::Cli;
use colored::Colorize;
use fern::Dispatch;
use std::{io::Read, process};

use commitfmt_cc::Message;
use commitfmt_config::{parse::CommitSettingsParser, settings::CommitParams};
use commitfmt_git::Repository;
use commitfmt_linter::{check::Check, violation::FixMode};
use report::{print_violation, report_violations};

#[derive(Debug, PartialEq, Eq)]
enum InputSource {
    Stdin,
    CommitEditMessage,
    None,
}

fn get_source(repo: Option<&Repository>) -> InputSource {
    if stdin::is_readable() {
        return InputSource::Stdin;
    }

    match repo {
        Some(repo) if repo.is_committing() => InputSource::CommitEditMessage,
        _ => InputSource::None,
    }
}

fn setup_logger(verbose: bool, no_color: bool) {
    let log_level = if verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    Dispatch::new()
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .expect("Unable to set up logger");

    if no_color {
        colored::control::set_override(false);
    }
}

fn handle_commit_range(
    repo: &Repository,
    from: &str,
    to: &str,
    params: &CommitParams,
) -> process::ExitCode {
    let commits = match repo.get_commits(from, to) {
        Ok(commits) => commits,
        Err(err) => {
            print_error!("Failed to get commits: {}", err);
            return process::ExitCode::FAILURE;
        }
    };

    let mut has_problems = false;
    let mut check = Check::new(&params.settings, params.rules.clone());

    for commit in commits {
        let message = match Message::parse(&commit.message) {
            Ok(message) => message,
            Err(_) => {
                print_error!("Failed to parse commit message");
                return process::ExitCode::FAILURE;
            }
        };

        check.lint(&message);
        if !check.report.violations.is_empty() {
            has_problems = true;
            let count = report_violations(check.report.violations.iter());
            print_error!("Commit {} has {} violations", commit.sha, count);
            check.report.violations.clear();
        }
    }

    if has_problems {
        process::ExitCode::FAILURE
    } else {
        process::ExitCode::SUCCESS
    }
}

fn handle_single_message(
    source: InputSource,
    repo: Option<&Repository>,
    params: &CommitParams,
    lint: bool,
) -> process::ExitCode {
    let mut input = String::new();

    match source {
        InputSource::Stdin => {
            if let Err(err) = std::io::stdin().read_to_string(&mut input) {
                print_error!("Failed to read stdin: {}", err);
                return process::ExitCode::FAILURE;
            }
        }
        InputSource::CommitEditMessage => {
            input = match repo.unwrap().read_commit_message() {
                Ok(msg) => msg,
                Err(err) => {
                    print_error!("Failed to read commit message: {}", err);
                    return process::ExitCode::FAILURE;
                }
            };
        }
        InputSource::None => {
            unreachable!();
        }
    };

    let mut message = match Message::parse(&input) {
        Ok(message) => message,
        Err(_) => {
            print_error!("Failed to parse commit message");
            return process::ExitCode::FAILURE;
        }
    };

    let mut check = Check::new(&params.settings, params.rules.clone());
    check.lint(&message);

    if lint {
        if check.report.violations.is_empty() {
            return process::ExitCode::SUCCESS;
        }
        let count = report_violations(check.report.violations.iter());
        print_error!("\n{}", format!("{} problems found", count));
        return process::ExitCode::FAILURE;
    }

    let mut unfixable_count: usize = 0;
    let message_ptr = &mut message;
    for violation_box in &check.report.violations {
        let violation = violation_box.as_ref();
        match violation.fix_mode() {
            FixMode::Unsafe => {
                if params.formatting.unsafe_fixes {
                    violation.fix(message_ptr).expect("Failed to fix violation");
                } else {
                    // TODO: add available fixes report
                    print_violation(violation);
                    unfixable_count += 1;
                }
            },
            FixMode::Safe => {
                violation.fix(message_ptr).expect("Failed to fix violation");
            },
            FixMode::Unfixable => {
                print_violation(violation);
                unfixable_count += 1;
            }
        }
    }

    if unfixable_count > 0 {
        // TODO: pluralize
        print_error!("\n{}", format!("{} unfixable problems found", unfixable_count));
        return process::ExitCode::FAILURE;
    }

    if source == InputSource::CommitEditMessage {
        print_debug!("Writing commit message");
        if let Err(err) = repo.unwrap().write_commit_message(&message.to_string()) {
            print_error!("Failed to write commit message: {}", err);
            return process::ExitCode::FAILURE;
        }
    } else if source == InputSource::Stdin {
        print!("{}", message);
    }

    process::ExitCode::SUCCESS
}

fn main() -> process::ExitCode {
    let cli = Cli::parse();
    setup_logger(cli.verbose, cli.no_color);

    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(_) => {
            print_error!("Unable to get current directory");
            return process::ExitCode::FAILURE;
        }
    };

    let repo = Repository::open(&cwd).ok();
    let params = match CommitParams::load(&cwd) {
        Ok(params) => params.unwrap_or_default(),
        Err(err) => {
            print_error!("Failed to load settings: {}", err);
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

        let repo = match Repository::open(&cwd) {
            Ok(repo) => repo,
            Err(err) => {
                print_error!("Failed to open repository: {}", err);
                return process::ExitCode::FAILURE;
            }
        };

        return handle_commit_range(&repo, from, to, &params);
    }

    let source = get_source(repo.as_ref());
    print_debug!("Input source: {:#?}", source);

    if source == InputSource::None {
        print_error!("Unable to determine input source\n");
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        return process::ExitCode::FAILURE;
    }

    handle_single_message(source, repo.as_ref(), &params, cli.lint)
}
