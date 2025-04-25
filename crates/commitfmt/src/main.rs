mod cli;
mod logging;
mod report;
mod stdin;

use clap::{CommandFactory, Parser};
use cli::Cli;
use colored::Colorize;
use commitfmt_format::append_footers;
use fern::Dispatch;
use log::info;
use std::{io::Read, process};

use commitfmt_cc::Message;
use commitfmt_config::{params::CommitParams, parse::CommitSettingsParser};
use commitfmt_git::Repository;
use commitfmt_linter::{check::Check, violation::FixMode};

use report::{print_violation, report_violations};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum InputSource {
    Stdin,
    CommitEditMessage,
    None,
}

fn get_source(repo: &Repository) -> InputSource {
    if stdin::is_readable() {
        return InputSource::Stdin;
    }

    if repo.is_committing() {
        return InputSource::CommitEditMessage;
    }

    InputSource::None
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
    let mut check = Check::new(&params.rules.settings, params.rules.set);

    for commit in commits {
        let Ok(message) = Message::parse(&commit.message) else {
            print_error!("Failed to parse commit message");
            return process::ExitCode::FAILURE;
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
    repo: &Repository,
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
            input = match repo.read_commit_message() {
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
    }

    let Ok(mut message) = Message::parse(&input) else {
        print_error!("Failed to parse commit message");
        return process::ExitCode::FAILURE;
    };

    let mut check = Check::new(&params.rules.settings, params.rules.set);
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
                if params.lint.unsafe_fixes {
                    violation.fix(message_ptr).expect("Failed to fix violation");
                } else {
                    // TODO: add available fixes report
                    print_violation(violation);
                    unfixable_count += 1;
                }
            }
            FixMode::Safe => {
                violation.fix(message_ptr).expect("Failed to fix violation");
            }
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

    if let Some(branch) = repo.get_branch_name() {
        match append_footers(&mut message, &params.footers.borrow(), &branch) {
            Ok(()) => {}
            Err(err) => {
                print_error!("Failed to append footers: {}", err);
                return process::ExitCode::FAILURE;
            }
        }
    }

    if source == InputSource::CommitEditMessage {
        print_debug!("Writing commit message");
        if let Err(err) = repo.write_commit_message(&message.to_string()) {
            print_error!("Failed to write commit message: {}", err);
            return process::ExitCode::FAILURE;
        }
    } else if source == InputSource::Stdin {
        info!("{}", message);
    }

    process::ExitCode::SUCCESS
}

fn main() -> process::ExitCode {
    let cli = Cli::parse();
    setup_logger(cli.verbose, cli.no_color);

    let Ok(cwd) = std::env::current_dir() else {
        print_error!("Unable to get current directory");
        return process::ExitCode::FAILURE;
    };

    let repo = match Repository::open(&cwd) {
        Ok(repo) => repo,
        Err(err) => {
            print_error!("Failed to open repository: {}", err);
            return process::ExitCode::FAILURE;
        }
    };

    let params = match CommitParams::load(&cwd) {
        Ok(params) => params.unwrap_or_default(),
        Err(err) => {
            print_error!("Failed to load settings: {}", err);
            return process::ExitCode::FAILURE;
        }
    };

    print_debug!("Params: {:#?}", params);

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

        return handle_commit_range(&repo, from, to, &params);
    }

    let source = get_source(&repo);
    print_debug!("Input source: {:#?}", source);

    if source == InputSource::None {
        print_error!("Unable to determine input source\n");
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        return process::ExitCode::FAILURE;
    }

    handle_single_message(source, &repo, &params, cli.lint)
}
