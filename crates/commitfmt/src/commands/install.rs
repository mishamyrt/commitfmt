use colored::Colorize;
use commitfmt_git::{is_git_available, HookType, Repository};
use commitfmt_hook::{Manager, HOOK_CONTENT};
use log::{debug, error, info};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io, process};

const README_URL: &str = "https://github.com/mishamyrt/commitfmt?tab=readme-ov-file";

fn write_hook(path: &Path) -> Result<(), io::Error> {
    fs::write(path, HOOK_CONTENT)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;

    Ok(())
}

fn install_hook(path: &Path) -> process::ExitCode {
    match write_hook(path) {
        Ok(()) => {
            info!("{}", "Hook successfully installed".bright_green());
            process::ExitCode::SUCCESS
        }
        Err(err) => {
            error!("Failed to write hook: {}", err);
            process::ExitCode::FAILURE
        }
    }
}

pub(crate) fn run_install(working_dir: &Path, force: bool) -> process::ExitCode {
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

    if force || !hook_path.exists() {
        return install_hook(&hook_path);
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
            let Some(guide_anchor) = runner.guide_anchor else {
                info!("{}", "Hook already installed".bright_green());
                return process::ExitCode::SUCCESS;
            };

            let guide_url = format!("{README_URL}#{guide_anchor}");
            info!(
                "{}",
                format!("Detected {}. You can use it in tandem with commitfmt!", runner.name)
                    .bright_green()
            );
            info!("{}", format!("Setup instructions: {guide_url}").yellow());
            info!("{}", "To replace the hook, run:");
            info!("{}", "  commitfmt install --force".cyan());
        }
        None => {
            info!("{}", "Detected unknown hook.".yellow());
            info!("{}", "To install the hook, run:");
            info!("{}", "  commitfmt install".cyan());
        }
    }

    process::ExitCode::SUCCESS
}
