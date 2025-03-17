use std::path::Path;
use std::process;

use commitfmt_cc::Message;
use commitfmt_config::parse::CommitSettingsParser;
use commitfmt_config::settings::CommitParams;
use commitfmt_linter::check::Check;
use commitfmt_linter::rules::Rule;
use colored::Colorize;
use log::info;

pub(crate) fn run_fix(_working_dir: &Path) -> process::ExitCode {
    process::ExitCode::SUCCESS
}

pub(crate) fn run_preview(working_dir: &Path, input: &str) -> process::ExitCode {
    let params = match CommitParams::load(working_dir) {
        Ok(params) => params.unwrap_or_default(),
        Err(err) => {
            info!("Failed to load settings: {}", err);
            return process::ExitCode::FAILURE;
        }
    };

    let Ok(message) = Message::parse(input) else {
        info!("Failed to parse commit message");
        return process::ExitCode::FAILURE;
    };

    let check = Check::new(params.settings, params.rules);
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
