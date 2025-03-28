use crate::{parse_toml::parse_toml, settings::CommitParams, ConfigError};

/// List of known config file names
const KNOWN_PATHS: &[&str] = &[
    ".commitfmt.toml",
    "commitfmt.toml",
    ".commitfmt.yaml",
    "commitfmt.yaml",
    ".commitfmt.yml",
    "commitfmt.yml",
];

/// Maximum size of the config file
/// If the file is larger than this, return an error.
const MAX_CONFIG_SIZE: u64 = 1_000_000;

/// Supported config file formats
pub enum Format {
    Toml,
    Yaml,
}

impl Format {
    /// Get the format from the file extension
    fn from_extension(extension: &std::ffi::OsStr) -> Option<Format> {
        match extension.to_str() {
            Some(ext) => match ext {
                "toml" => Some(Format::Toml),
                "yaml" | "yml" => Some(Format::Yaml),
                _ => None,
            },
            _ => None,
        }
    }
}

pub trait CommitSettingsParser {
    fn from_str(format: Format, data: &str) -> Result<CommitParams, ConfigError>;
    fn from_file(path: &std::path::Path) -> Result<CommitParams, ConfigError>;
    fn load(dir_path: &std::path::Path) -> Result<Option<CommitParams>, ConfigError>;
    fn exists(dir_path: &std::path::Path) -> bool;
}

impl CommitSettingsParser for CommitParams {
    fn from_str(format: Format, data: &str) -> Result<CommitParams, ConfigError> {
        match format {
            Format::Toml => parse_toml(data),
            Format::Yaml => Err(ConfigError::UnsupportedFileType("yaml".to_owned())),
        }
    }

    fn from_file(path: &std::path::Path) -> Result<CommitParams, ConfigError> {
        if std::fs::metadata(path)?.len() > MAX_CONFIG_SIZE {
            return Err(ConfigError::FileTooLarge);
        }

        let Some(extension) = path.extension() else {
            return Err(ConfigError::UnsupportedFileType("unknown".to_owned()));
        };
        let Some(format) = Format::from_extension(extension) else {
            return Err(ConfigError::UnsupportedFileType(
                extension.to_str().unwrap_or("unknown").to_owned(),
            ));
        };
        let data = std::fs::read_to_string(path).map_err(ConfigError::IOError)?;

        Self::from_str(format, &data)
    }

    fn load(dir_path: &std::path::Path) -> Result<Option<CommitParams>, ConfigError> {
        for known_path in KNOWN_PATHS {
            let path = dir_path.join(known_path);
            if path.exists() {
                let settings = Self::from_file(&path)?;
                return Ok(Some(settings));
            }
        }
        Ok(None)
    }

    fn exists(dir_path: &std::path::Path) -> bool {
        for known_path in KNOWN_PATHS {
            let path = dir_path.join(known_path);
            if path.exists() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use commitfmt_linter::{rule_set::RuleSet, rules};

    use crate::settings::FormattingSettings;

    use super::*;

    #[test]
    fn test_from_str() {
        let config = "
[body]
max-line-length = 80

[formatting]
unsafe-fixes = true";
        let mut settings = rules::Settings::default();
        settings.body.max_line_length = 80;

        let expected = CommitParams {
            formatting: FormattingSettings {
                unsafe_fixes: true,
                footers: RefCell::new(vec![]),
            },
            rules: RuleSet::default()
                .union(RuleSet::from_rules(&[rules::Rule::BodyMaxLineLength])),
            settings,
        };

        let result = CommitParams::from_str(Format::Toml, config).unwrap();
        assert_eq!(result, expected);
    }
}
