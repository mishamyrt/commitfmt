// include!(concat!(env!("OUT_DIR"), "/hook_content.rs"));

// use colored::Colorize;
// use log::{debug, error, info};
// use std::os::unix::fs::PermissionsExt;
// use std::path::Path;
// use std::{env, fs, io, process};

// use commitfmt::git;
// use commitfmt::runner;

// const README_URL: &str = "https://github.com/mishamyrt/commitfmt?tab=readme-ov-file";

// pub fn write_hook(path: &Path) -> Result<(), io::Error> {
//     fs::write(path, HOOK_CONTENT)?;
//     fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;

//     Ok(())
// }

// pub fn install_hook(path: &Path) -> process::ExitCode {
//     match write_hook(path) {
//         Ok(()) => {
//             info!("{}", "Hook successfully installed".bright_green());
//             process::ExitCode::SUCCESS
//         }
//         Err(err) => {
//             error!("Failed to write hook: {}", err);
//             process::ExitCode::FAILURE
//         }
//     }
// }

// pub fn run_install(working_dir: &Path, force: bool) -> process::ExitCode {
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

//     if force {
//         return install_hook(&hook_path);
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
//                 info!("{}", "Hook already installed".bright_green());
//                 return process::ExitCode::SUCCESS;
//             }

//             let guide_url = format!("{}#{}", README_URL, runner.guide_anchor);
//             info!("{}", format!("Detected {}. You can use it in tandem with commitfmt!", runner.name).bright_green());
//             info!("{}", format!("Setup instructions: {}", guide_url).yellow());
//             info!("{}", "To replace the hook, run:");
//             info!("{}", "  commitfmt install --force".cyan());
//         }
//         None => {
//             info!("{}", "Detected unknown hook.".yellow());
//             info!("{}", "To install the hook, run:");
//             info!("{}", "  commitfmt install".cyan());
//         }
//     }

//     process::ExitCode::SUCCESS
// }
