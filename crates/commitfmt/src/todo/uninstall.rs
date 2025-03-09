// use colored::Colorize;
// use log::{debug, error, info};
// use std::{fs, path::Path, process};

// use commitfmt::{git, runner};

// pub fn run_uninstall(working_dir: &Path, force: bool) -> process::ExitCode {
//     if !git::is_available() {
//         error!("{}", "Git is not available".bright_red());
//         return process::ExitCode::FAILURE;
//     }
//     let repo = match git::Repository::open(working_dir) {
//         Ok(repo) => repo,
//         Err(err) => {
//             error!("Failed to open repository: {}", err);
//             return process::ExitCode::FAILURE;
//         }
//     };

//     let hook_path = match repo.hook_path(git::HookType::PrepareCommitMsg) {
//         Ok(hooks_path) => hooks_path,
//         Err(_) => {
//             error!("Hook path not found");
//             return process::ExitCode::FAILURE;
//         }
//     };
//     debug!("Hook path: {}", hook_path.to_str().unwrap());

//     if !hook_path.exists() {
//         if force {
//             return process::ExitCode::SUCCESS;
//         }
//         error!("{}", "Hook is not found".bright_red());
//         return process::ExitCode::FAILURE;
//     }

//     if force {
//         return uninstall_hook(&hook_path);
//     }

//     let runner = match runner::detect_runner(&hook_path) {
//         Ok(runner) => runner,
//         Err(err) => {
//             error!("Failed to detect hook runner: {}", err);
//             return process::ExitCode::FAILURE;
//         }
//     };

//     match runner {
//         Some(runner) => {
//             if runner.guide_anchor.is_empty() {
//                 return uninstall_hook(&hook_path);
//             }
//             info!("{}", "Detected third party hook.".yellow());
//             info!("{}", "To uninstall the hook, run:");
//             info!("{}", "  commitfmt uninstall --force".bright_cyan());
//             process::ExitCode::SUCCESS
//         }
//         None => {
//             error!("Hook is not found");
//             process::ExitCode::FAILURE
//         }
//     }
// }

// pub fn uninstall_hook(path: &Path) -> process::ExitCode {
//     match fs::remove_file(path) {
//         Ok(()) => {
//             info!("{}", "Hook successfully uninstalled".bright_green());
//             process::ExitCode::SUCCESS
//         }
//         Err(err) => {
//             error!("Failed to remove hook: {}", err);
//             process::ExitCode::FAILURE
//         }
//     }
// }
