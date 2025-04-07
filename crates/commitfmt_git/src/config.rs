pub fn get_trailer_separators() -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("--config")
        .arg("trailer.separators")
        .output()
        .ok()?;
    let content = String::from_utf8_lossy(&output.stdout);

    Some(content.to_string())
}
