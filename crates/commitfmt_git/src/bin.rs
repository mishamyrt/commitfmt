/// Checks if git is available in PATH
pub fn is_git_available() -> bool {
    let output = std::process::Command::new("git").arg("--version").output().ok();
    output.is_some()
}
