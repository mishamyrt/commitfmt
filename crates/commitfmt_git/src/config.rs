pub fn get_trailer_separators<'c>() -> Option<String> {
    let Some(output) = std::process::Command::new("git")
        .arg("--config")
        .arg("trailer.separators")
        .output()
        .ok()
    else {
        return None;
    };
    let content = String::from_utf8_lossy(&output.stdout);

    return Some(content.to_string());
}
