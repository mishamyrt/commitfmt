use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]

/// Git hook manager
pub struct Manager {
    /// Name of the manager
    pub name: &'static str,

    /// Anchor for the guide (README.md)
    pub guide_anchor: Option<&'static str>,

    /// Variable name that is used to be in script
    indicator: &'static str,
}

impl Manager {
    pub fn assert(&self, content: &str) -> bool {
        content.contains(self.indicator)
    }
}

const MANAGERS: &[Manager] = &[
    Manager { name: "commitfmt", guide_anchor: None, indicator: "COMMITFMT_BIN" },
    Manager { name: "Lefthook", guide_anchor: Some("lefthook"), indicator: "LEFTHOOK_BIN" },
];

/// Returns the manager that is used for the given content
pub fn detect(content: &str) -> Option<Manager> {
    for manager in MANAGERS {
        if manager.assert(content) {
            return Some(*manager);
        }
    }
    None
}

/// Returns the manager that is used for the given path
pub fn detect_from_path(path: &PathBuf) -> Result<Option<Manager>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(detect(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        let content = "COMMITFMT_BIN";
        assert!(detect(content).unwrap().name == "commitfmt");

        let content = "LEFTHOOK_BIN";
        assert!(detect(content).unwrap().name == "Lefthook");
    }
}
