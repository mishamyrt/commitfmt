use colored::Colorize;
use commitfmt_git::{is_git_available, HookType, Repository};
use commitfmt_hook::Manager;
use log::{debug, error, info};
use std::{fs, path::Path, process};

pub(crate) fn run_uninstall(working_dir: &Path, force: bool) -> process::ExitCode {
    if !is_git_available() {
        error!("{}", "Git is not available".bright_red());
        return process::ExitCode::FAILURE;
    }
    let repo = match Repository::open(working_dir) {
        Ok(repo) => repo,
        Err(err) => {
            error!("Failed to open repository: {}", err);
            return process::ExitCode::FAILURE;
        }
    };

    let Ok(hook_path) = repo.hook_path(HookType::PrepareCommitMsg) else {
        error!("Hook path not found");
        return process::ExitCode::FAILURE;
    };
    debug!("Hook path: {}", hook_path.to_str().unwrap());

    if !hook_path.exists() {
        if force {
            return process::ExitCode::SUCCESS;
        }
        error!("{}", "Hook is not found".bright_red());
        return process::ExitCode::FAILURE;
    }

    if force {
        return uninstall_hook(&hook_path);
    }

    let runner = match Manager::detect_from_path(&hook_path) {
        Ok(runner) => runner,
        Err(err) => {
            error!("Failed to detect hook runner: {}", err);
            return process::ExitCode::FAILURE;
        }
    };

    match runner {
        Some(runner) => {
            if runner.guide_anchor.is_none() {
                return uninstall_hook(&hook_path);
            };

            info!("{}", "Detected third party hook.".yellow());
            info!("{}", "To uninstall the hook, run:");
            info!("{}", "  commitfmt uninstall --force".bright_cyan());
            process::ExitCode::SUCCESS
        }
        None => {
            error!("Hook is not found");
            process::ExitCode::FAILURE
        }
    }
}

pub(crate) fn uninstall_hook(path: &Path) -> process::ExitCode {
    match fs::remove_file(path) {
        Ok(()) => {
            info!("{}", "Hook successfully uninstalled".bright_green());
            process::ExitCode::SUCCESS
        }
        Err(err) => {
            error!("Failed to remove hook: {}", err);
            process::ExitCode::FAILURE
        }
    }
}
