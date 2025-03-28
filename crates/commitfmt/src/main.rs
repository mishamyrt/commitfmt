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
use report::report_violations;

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
        Some(repo) => {
            if repo.is_committing() {
                InputSource::CommitEditMessage
            } else {
                InputSource::None
            }
        }
        None => InputSource::None,
    }
}

fn main() -> process::ExitCode {
    let cli = Cli::parse();

    let log_level = if cli.verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    Dispatch::new()
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .expect("Unable to set up logger");
    if cli.no_color {
        colored::control::set_override(false);
    }

    let Ok(cwd) = std::env::current_dir() else {
        print_error!("Unable to get current directory");

        return process::ExitCode::FAILURE;
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

        let to = match &cli.to {
            Some(to) => to,
            None => &String::from("HEAD"),
        };

        let from = cli.from.as_ref().unwrap();

        let repo = match Repository::open(&cwd) {
            Ok(repo) => repo,
            Err(err) => {
                print_error!("Failed to open repository: {}", err);

                return process::ExitCode::FAILURE;
            }
        };

        let commits = match repo.get_commits(from, to) {
            Ok(commits) => commits,
            Err(err) => {
                print_error!("Failed to get commits: {}", err);

                return process::ExitCode::FAILURE;
            }
        };

        let mut has_problems = false;
        let mut check = Check::new(params.settings, params.rules);
        for commit in commits {
            let Ok(message) = Message::parse(&commit.message) else {
                print_error!("Failed to parse commit message");

                return process::ExitCode::FAILURE;
            };

            check.run(&message);
            if !check.report.violations.is_empty() {
                has_problems = true;
                let count = report_violations(check.report.violations.iter());
                print_error!("Commit {} has {} violations", commit.sha, count);

                check.report.violations.clear();
            }
        }

        if !has_problems {
            return process::ExitCode::SUCCESS;
        } else {
            return process::ExitCode::FAILURE;
        }
    }

    let source = get_source(repo.as_ref());
    print_debug!("Input source: {:#?}", source);
    if source == InputSource::None {
        print_error!("Unable to determine input source\n");
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();

        return process::ExitCode::FAILURE;
    }

    let mut input = String::new();
    match source {
        InputSource::Stdin => {
            match std::io::stdin().read_to_string(&mut input) {
                Ok(_) => {}
                Err(err) => {
                    print_error!("Failed to read stdin: {}", err);

                    return process::ExitCode::FAILURE;
                }
            };
        }
        InputSource::CommitEditMessage => {
            input = repo.as_ref().unwrap().read_commit_message().unwrap();
        }
        InputSource::None => {
            unreachable!();
        }
    };

    let Ok(message) = Message::parse(&input) else {
        print_error!("Failed to parse commit message");

        return process::ExitCode::FAILURE;
    };

    let mut check = Check::new(params.settings, params.rules);
    check.run(&message);

    if cli.lint {
        if check.report.violations.is_empty() {
            return process::ExitCode::SUCCESS;
        }
        let count = report_violations(check.report.violations.iter());

        print_error!("\n{}", format!("{} problems found", count));

        return process::ExitCode::FAILURE;
    }

    let unfixable = check.report.violations.iter().filter(|violation_box| {
        let violation = violation_box.as_ref();
        // TODO: add FixMode::Unsafe handling
        violation.fix_mode() == FixMode::Unfixable
    });

    let unfixable_count: usize = report_violations(unfixable);

    if unfixable_count > 0 {
        // TODO: pluralize
        print_error!("\n{}", format!("{} unfixable problems found", unfixable_count));
        return process::ExitCode::FAILURE;
    }

    if source == InputSource::CommitEditMessage {
        print_debug!("Writing commit message");
        match repo.as_ref().unwrap().write_commit_message(&message.to_string()) {
            Ok(_) => {}
            Err(err) => {
                print_error!("Failed to write commit message: {}", err);
                return process::ExitCode::FAILURE;
            }
        };
    } else if source == InputSource::Stdin {
        print!("{}", message);
    }

    process::ExitCode::SUCCESS
}
