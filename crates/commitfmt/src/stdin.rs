use std::process;
use colored::Colorize;
use commitfmt_cc::message::Message;
use commitfmt_config::settings::CommitSettings;
use commitfmt_config::parse::CommitSettingsParser;
use commitfmt_linter::check::Check;
use commitfmt_linter::rules::Rule;
use log::info;

pub(crate) fn run_stdin(input: &str, dir_path: &std::path::Path) -> process::ExitCode {
    let commit_settings = match CommitSettings::load(dir_path) {
        Ok(settings) => settings,
        Err(err) => {
            info!("Failed to load settings: {}", err);
            return process::ExitCode::FAILURE;
        }
    };

    let Ok(message) = Message::parse(input) else {
        info!("Failed to parse commit message");
        return process::ExitCode::FAILURE;
    };

    let check = Check::new(commit_settings.settings, commit_settings.rules);

    check.run(&message);

    for violation_box in check.violations_ref().borrow().iter() {
        let violation = violation_box.as_ref();
        let Some(rule) = Rule::from_violation(violation) else {
            panic!("Failed to get rule from violation");
        };

        let line = format!("- {} {}", violation.message(), rule.as_display().dimmed());
        info!("{}", line);
    }

    process::ExitCode::SUCCESS
}
